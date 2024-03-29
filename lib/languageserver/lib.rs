mod envelope;
mod loaded_file;
mod models;
mod triggers;
mod workspace;

use std::collections::{BTreeSet, Bound, HashMap};
use std::fmt;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::ops::DerefMut;
use std::path::Path;
use std::result;
use std::sync::{Arc, Mutex};
use url::Url;

use envelope::RequestId;
use reproto_core::errors::Result;
use reproto_core::{Diagnostic, Encoding, Filesystem, RealFilesystem, Reported, Rope, Source};
use serde::Deserialize;

use crate::loaded_file::LoadedFile;
use crate::models::{Completion, Jump, Range, RenameResult};
use crate::workspace::Workspace;
use crate::ContentType::*;

#[derive(Debug)]
enum ContentType {
    JsonRPC,
}

#[derive(Debug)]
struct Headers {
    content_type: ContentType,
    content_length: u32,
}

impl Headers {
    pub fn new() -> Self {
        Self {
            content_type: JsonRPC,
            content_length: 0u32,
        }
    }

    fn clear(&mut self) {
        self.content_type = JsonRPC;
        self.content_length = 0;
    }
}

/// Reads input stream for server.
struct InputReader<R> {
    reader: R,
    buffer: Vec<u8>,
}

impl<R> InputReader<R>
where
    R: BufRead,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
        }
    }

    fn next_line<'a>(&'a mut self) -> Result<Option<&'a [u8]>> {
        self.buffer.clear();
        self.reader.read_until('\n' as u8, &mut self.buffer)?;

        if self.buffer.is_empty() {
            return Ok(None);
        }

        Ok(Some(trim(&self.buffer)))
    }
}

impl<R> Read for InputReader<R>
where
    R: BufRead,
{
    fn read(&mut self, buf: &mut [u8]) -> result::Result<usize, io::Error> {
        self.reader.read(buf)
    }
}

/// A language server channel, taking care of locking and sending notifications.
struct Channel<W> {
    /// Request id allocation.
    next_id: Arc<Mutex<u64>>,
    /// A writer and buffer pair.
    output: Arc<Mutex<(Vec<u8>, W)>>,
}

impl<W> Clone for Channel<W> {
    fn clone(&self) -> Self {
        Channel {
            next_id: Arc::clone(&self.next_id),
            output: Arc::clone(&self.output),
        }
    }
}

impl<W> Channel<W> {
    pub fn new(writer: W) -> Self {
        Self {
            next_id: Arc::new(Mutex::new(0u64)),
            output: Arc::new(Mutex::new((Vec::new(), writer))),
        }
    }
}

impl<W> Channel<W>
where
    W: Write,
{
    /// Send a complete frame.
    fn send_frame<T>(&self, frame: T) -> Result<()>
    where
        T: fmt::Debug + serde::Serialize,
    {
        log::debug!("send frame: {:#?}", frame);

        let mut guard = self.output.lock().map_err(|_| "lock poisoned")?;
        let &mut (ref mut buffer, ref mut writer) = guard.deref_mut();

        buffer.clear();
        json::to_writer(&mut *buffer, &frame)?;

        write!(writer, "Content-Length: {}\r\n\r\n", buffer.len())?;
        writer.write_all(buffer)?;
        writer.flush()?;

        Ok(())
    }

    /// Send a notification.
    fn notification<N>(&self, params: N::Params) -> Result<()>
    where
        N: ty::notification::Notification,
        N::Params: fmt::Debug,
    {
        let envelope = envelope::NotificationMessage::<N::Params> {
            jsonrpc: envelope::V2,
            method: N::METHOD.to_string(),
            params: Some(params),
        };

        self.send_frame(envelope)
    }

    /// Send a request.
    fn request<R>(&self, params: R::Params) -> Result<RequestId>
    where
        R: ty::request::Request,
        R::Params: fmt::Debug,
    {
        let id = {
            let mut next_id = self.next_id.lock().map_err(|_| "id allocation poisoned")?;
            let id = *next_id;
            *next_id = id + 1;
            RequestId::Number(id)
        };

        let envelope = envelope::RequestMessage::<R::Params> {
            jsonrpc: envelope::V2,
            id: Some(id.clone()),
            method: R::METHOD.to_string(),
            params: params,
        };

        self.send_frame(envelope)?;
        Ok(id)
    }

    /// Send a response message.
    fn send<T>(&self, request_id: Option<RequestId>, message: T) -> Result<()>
    where
        T: fmt::Debug + serde::Serialize,
    {
        let envelope = envelope::ResponseMessage::<T, ()> {
            jsonrpc: envelope::V2,
            id: request_id,
            result: Some(message),
            error: None,
        };

        self.send_frame(envelope)
    }

    /// Send an error.
    fn send_error<D>(
        &self,
        request_id: Option<RequestId>,
        error: envelope::ResponseError<D>,
    ) -> Result<()>
    where
        D: fmt::Debug + serde::Serialize,
    {
        let envelope = envelope::ResponseMessage::<(), D> {
            jsonrpc: envelope::V2,
            id: request_id,
            result: None,
            error: Some(error),
        };

        self.send_frame(envelope)
    }
}

pub struct Logger<L>
where
    L: Send,
{
    log: Mutex<L>,
}

impl<L> Logger<L>
where
    L: Send,
{
    pub fn new(log: L) -> Self {
        Self {
            log: Mutex::new(log),
        }
    }
}

impl<L> log::Log for Logger<L>
where
    L: Send + Write,
{
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut lock = self.log.lock().expect("poisoned lock");
            writeln!(lock, "{}: {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {}
}

/// Logger that sends logs as notifications to client.
struct NotificationLogger<W>
where
    W: Send,
{
    channel: Channel<W>,
}

impl<W> NotificationLogger<W>
where
    W: Send,
{
    pub fn new(channel: Channel<W>) -> Self {
        Self { channel }
    }
}

impl<W> log::Log for NotificationLogger<W>
where
    W: Send + Write,
{
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        use log::Level::*;

        if !self.enabled(record.metadata()) {
            return;
        }

        let typ = match record.level() {
            Error => ty::MessageType::ERROR,
            Warn => ty::MessageType::WARNING,
            Info => ty::MessageType::INFO,
            _ => ty::MessageType::LOG,
        };

        let notification = ty::LogMessageParams {
            typ,
            message: record.args().to_string(),
        };

        self.channel
            .notification::<ty::notification::LogMessage>(notification)
            .expect("failed to send notification");
    }

    fn flush(&self) {}
}

pub fn server<L, R, W>(log: Option<L>, reader: R, writer: W, level: log::LevelFilter) -> Result<()>
where
    L: 'static + Send + Write,
    R: Read,
    W: 'static + Send + Write,
{
    let channel = Channel::new(writer);

    if let Some(log) = log {
        let logger = Logger::new(log);

        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(level);
    } else {
        log::set_boxed_logger(Box::new(NotificationLogger::new(channel.clone())))?;
        log::set_max_level(level);
    }

    match try_server(reader, channel) {
        Err(e) => {
            log::error!("error: {}", e.display());

            for cause in e.causes().skip(1) {
                log::error!("caused by: {}", cause.display());
            }

            return Err(e);
        }
        Ok(()) => {
            return Ok(());
        }
    }
}

fn try_server<R, W>(reader: R, channel: Channel<W>) -> Result<()>
where
    R: Read,
    W: Send + Write,
{
    let mut server = Server::new(reader, channel);
    server.run()?;
    Ok(())
}

/// Trim the string from whitespace.
fn trim(data: &[u8]) -> &[u8] {
    let s = data
        .iter()
        .position(|b| *b != b'\n' && *b != b'\r' && *b != b' ')
        .unwrap_or(data.len());

    let data = &data[s..];

    let e = data
        .iter()
        .rev()
        .position(|b| *b != b'\n' && *b != b'\r' && *b != b' ')
        .map(|p| data.len() - p)
        .unwrap_or(0usize);

    &data[..e]
}

/// Server abstraction
struct Server<R, W> {
    workspace: Option<Workspace>,
    headers: Headers,
    reader: InputReader<BufReader<R>>,
    channel: Channel<W>,
    /// Filesystem abstraction.
    fs: RealFilesystem,
    /// Expected responses.
    expected: HashMap<RequestId, Expected>,
    /// Built-in types.
    built_ins: Vec<&'static str>,
}

impl<R, W> Server<R, W>
where
    R: Read,
    W: Write,
{
    pub fn new(reader: R, channel: Channel<W>) -> Self {
        Self {
            workspace: None,
            headers: Headers::new(),
            reader: InputReader::new(BufReader::new(reader)),
            channel,
            fs: RealFilesystem::new(),
            expected: HashMap::new(),
            built_ins: vec![
                "string", "bytes", "u32", "u64", "i32", "i64", "float", "double", "datetime", "any",
            ],
        }
    }

    /// Read headers.
    fn read_headers(&mut self) -> Result<bool> {
        self.headers.clear();

        loop {
            let line = self.reader.next_line()?;

            let line = match line {
                Some(line) => line,
                None => return Ok(false),
            };

            if line == b"" {
                break;
            }

            let mut parts = line.splitn(2, |b| *b == b':');

            let (key, value) = match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => (trim(key), trim(value)),
                out => {
                    return Err(format!("bad header: {:?}", out).into());
                }
            };

            match key {
                b"Content-Type" => match value {
                    b"application/vscode-jsonrpc; charset=utf-8" => {
                        self.headers.content_type = JsonRPC;
                    }
                    value => {
                        return Err(format!("bad value: {:?}", value).into());
                    }
                },
                b"Content-Length" => {
                    let value = ::std::str::from_utf8(value)
                        .map_err(|e| format!("bad content-length: {:?}: {}", value, e))?;

                    let value = value
                        .parse::<u32>()
                        .map_err(|e| format!("bad content-length: {}: {}", value, e))?;

                    self.headers.content_length = value;
                }
                key => {
                    return Err(format!("bad header: {:?}", key).into());
                }
            }
        }

        Ok(true)
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            if !self.read_headers()? {
                break;
            }

            if self.headers.content_length == 0 {
                continue;
            }

            match self.headers.content_type {
                JsonRPC => {
                    let message: json::Value = {
                        let reader = (&mut self.reader).take(self.headers.content_length as u64);
                        json::from_reader(reader)
                            .map_err(|e| format!("failed to deserialize message: {}", e))?
                    };

                    // requests
                    if message.get("method").is_some() {
                        self.handle_request(message)?;
                    } else {
                        self.handle_response(message)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle a request.
    fn handle_request(&mut self, message: json::Value) -> Result<()> {
        let request = envelope::RequestMessage::<json::Value>::deserialize(message)
            .map_err(|e| format!("failed to deserialize request: {}", e))?;

        // in case we need it to report errors.
        let id = request.id.clone();

        log::debug!("received: {:#?}", request);

        if let Err(e) = self.try_handle_request(request) {
            self.channel.send_error(
                id,
                envelope::ResponseError {
                    code: envelope::Code::InternalError,
                    message: e.display().to_string(),
                    data: Some(()),
                },
            )?;
        }

        Ok(())
    }

    /// Inner handler which is guarded against errors.
    ///
    /// Any error returned here will result in an error being sent _back_ to the language client as
    /// a ResponseError.
    fn try_handle_request(&mut self, request: envelope::RequestMessage<json::Value>) -> Result<()> {
        match request.method.as_str() {
            "initialize" => {
                let params = ty::InitializeParams::deserialize(request.params)?;
                self.initialize(request.id, params)?;
            }
            "initialized" => {
                let params = ty::InitializedParams::deserialize(request.params)?;
                self.initialized(params)?;
            }
            "shutdown" => {
                self.shutdown()?;
            }
            "textDocument/didChange" => {
                let params = ty::DidChangeTextDocumentParams::deserialize(request.params)?;
                self.text_document_did_change(params)?;
            }
            "textDocument/didOpen" => {
                let params = ty::DidOpenTextDocumentParams::deserialize(request.params)?;
                self.text_document_did_open(params)?;
            }
            "textDocument/didClose" => {
                let params = ty::DidCloseTextDocumentParams::deserialize(request.params)?;
                self.text_document_did_close(params)?;
            }
            "textDocument/didSave" => {
                let params = ty::DidSaveTextDocumentParams::deserialize(request.params)?;
                self.text_document_did_save(params)?;
            }
            "textDocument/completion" => {
                let params = ty::CompletionParams::deserialize(request.params)?;
                self.text_document_completion(request.id, params)?;
            }
            "textDocument/definition" => {
                let params = ty::TextDocumentPositionParams::deserialize(request.params)?;
                self.text_document_definition(request.id, params)?;
            }
            "textDocument/rename" => {
                let params = ty::RenameParams::deserialize(request.params)?;
                self.text_document_rename(request.id, params)?;
            }
            "textDocument/documentSymbol" => {
                let params = ty::DocumentSymbolParams::deserialize(request.params)?;
                self.text_document_document_symbol(request.id, params)?;
            }
            "textDocument/references" => {
                let params = ty::ReferenceParams::deserialize(request.params)?;
                self.text_document_references(request.id, params)?;
            }
            "workspace/symbol" => {
                let params = ty::WorkspaceSymbolParams::deserialize(request.params)?;
                self.workspace_symbol(request.id, params)?;
            }
            "workspace/didChangeConfiguration" => {
                let params = ty::DidChangeConfigurationParams::deserialize(request.params)?;
                self.workspace_did_change_configuration(request.id, params)?;
            }
            "completionItem/resolve" => {
                let params = ty::CompletionItem::deserialize(request.params)?;
                self.completion_item_resolve(params)?;
            }
            "$/cancelRequest" => {
                // ignore
            }
            method => {
                log::error!("unsupported method: {}", method);

                self.channel.send_error(
                    request.id,
                    envelope::ResponseError {
                        code: envelope::Code::MethodNotFound,
                        message: "No such method".to_string(),
                        data: Some(()),
                    },
                )?;
            }
        }

        Ok(())
    }

    /// Handle a response.
    fn handle_response(&mut self, message: json::Value) -> Result<()> {
        // responses
        let response = envelope::ResponseMessage::<json::Value, json::Value>::deserialize(message)
            .map_err(|e| format!("failed to deserialize request: {}", e))?;

        if let Some(_) = response.error {
            // TODO: handle error
            return Ok(());
        }

        let id = match response.id {
            Some(ref id) => id,
            None => return Ok(()),
        };

        let expected = match self.expected.remove(&id) {
            Some(expected) => expected,
            None => {
                log::debug!("no handle for id: {:?}", id);
                return Ok(());
            }
        };

        log::debug!("response: {:?} {:#?}", expected, response);

        match expected {
            Expected::ProjectInit => {
                let result = match response.result {
                    Some(result) => result,
                    None => return Ok(()),
                };

                let response = Option::<ty::MessageActionItem>::deserialize(result)?;
                self.handle_project_init(response)?;
            }
            Expected::ProjectAddMissing => {
                let result = match response.result {
                    Some(result) => result,
                    None => return Ok(()),
                };

                let response = Option::<ty::MessageActionItem>::deserialize(result)?;
                self.handle_project_add_missing(response)?;
            }
        }

        Ok(())
    }

    /// Handle the response of `reproto/projectInit`.
    fn handle_project_init(&mut self, response: Option<ty::MessageActionItem>) -> Result<()> {
        let response = match response {
            Some(response) => response,
            None => return Ok(()),
        };

        if let Some(workspace) = &mut self.workspace {
            let handle = self.fs.open_root(Some(&workspace.root_path))?;

            if response.title == "Initialize project" {
                log::info!("Initializing Project!");
                workspace.initialize(&*handle)?;

                let manifest_url = workspace.manifest_url()?;

                self.channel.notification::<OpenUrl>(manifest_url)?;
            }
        }

        Ok(())
    }

    /// Handle the response of `reproto/projectAddMissing`.
    fn handle_project_add_missing(
        &mut self,
        response: Option<ty::MessageActionItem>,
    ) -> Result<()> {
        let response = match response {
            Some(response) => response,
            None => return Ok(()),
        };

        if let Some(workspace) = &mut self.workspace {
            if response.title == "Open project manifest" {
                let manifest_url = workspace.manifest_url()?;

                self.channel.notification::<OpenUrl>(manifest_url)?;
            }
        }

        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }

    /// Handler for `initialize`.
    fn initialize(
        &mut self,
        request_id: Option<RequestId>,
        params: ty::InitializeParams,
    ) -> Result<()> {
        // TODO: make use of root_uri instead.
        if let Some(path) = params.root_uri.as_ref().map(|uri| uri.path()) {
            let path = Path::new(path);

            let path = path
                .canonicalize()
                .map_err(|_| format!("could not canonicalize root path: {}", path.display()))?;

            let workspace = Workspace::new(Box::new(self.fs.clone()), path);
            self.workspace = Some(workspace);
        }

        let result = ty::InitializeResult {
            capabilities: ty::ServerCapabilities {
                text_document_sync: Some(ty::TextDocumentSyncCapability::Kind(
                    ty::TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(ty::CompletionOptions {
                    trigger_characters: Some(vec![":".into(), ".".into()]),
                    ..ty::CompletionOptions::default()
                }),
                definition_provider: Some(ty::OneOf::Left(true)),
                rename_provider: Some(ty::OneOf::Left(true)),
                document_symbol_provider: Some(ty::OneOf::Left(true)),
                workspace_symbol_provider: Some(ty::OneOf::Left(true)),
                references_provider: Some(ty::OneOf::Left(true)),
                ..ty::ServerCapabilities::default()
            },
            server_info: Some(ty::ServerInfo {
                name: String::from("reproto"),
                version: None,
            }),
        };

        self.channel.send(request_id, result)?;
        Ok(())
    }

    /// Handler for `initialized`.
    fn initialized(&mut self, _params: ty::InitializedParams) -> Result<()> {
        if let Some(workspace) = &mut self.workspace {
            log::debug!("loading project: {}", workspace.root_path.display());
            workspace.reload()?;
        }

        self.send_workspace_diagnostics()?;
        Ok(())
    }

    /// Handler for `workspace/symbol`.
    fn workspace_symbol(
        &mut self,
        request_id: Option<RequestId>,
        params: ty::WorkspaceSymbolParams,
    ) -> Result<()> {
        let query = params.query;

        let mut symbols = Vec::new();

        if let Some(workspace) = &mut self.workspace {
            for file in workspace.files() {
                Self::populate_symbols(&mut symbols, file, Some(query.as_str()))?;
            }
        }

        self.channel.send(request_id, Some(symbols))?;
        Ok(())
    }

    /// Populate symbol information from file.
    fn populate_symbols(
        symbols: &mut Vec<ty::SymbolInformation>,
        file: &LoadedFile,
        query: Option<&str>,
    ) -> Result<()> {
        let query = query.map(|q| {
            q.split(|c: char| c.is_whitespace() || !c.is_alphanumeric())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_lowercase())
                .collect::<Vec<_>>()
        });

        for (key, syms) in &file.symbols {
            for s in syms {
                if let Some(query) = query.as_ref() {
                    let mut k = key
                        .iter()
                        .map(|p| p.to_lowercase())
                        .collect::<BTreeSet<_>>();

                    k.insert(s.name.to_lowercase());

                    let matches = query.iter().all(|q| {
                        let range = (Bound::Included(q.to_string()), Bound::Unbounded);

                        k.range(range)
                            .next()
                            .map(|r| r.starts_with(q))
                            .unwrap_or(false)
                    });

                    if !matches {
                        continue;
                    }
                }

                let mut path = key.clone();
                path.push(s.name.clone());

                let range = convert_range(s.range);

                let location = ty::Location {
                    uri: s.url.clone(),
                    range,
                };

                #[allow(deprecated)]
                symbols.push(ty::SymbolInformation {
                    name: path.join("::"),
                    kind: ty::SymbolKind::CLASS,
                    location: location,
                    container_name: Some(file.package.to_string()),
                    deprecated: None,
                    tags: None,
                });
            }
        }

        Ok(())
    }

    /// Handler for `textDocument/documentSymbol`.
    fn text_document_document_symbol(
        &mut self,
        request_id: Option<RequestId>,
        params: ty::DocumentSymbolParams,
    ) -> Result<()> {
        let url = params.text_document.uri;

        let mut symbols = Vec::new();

        if let Some(workspace) = &mut self.workspace {
            if let Some(file) = workspace.file(&url) {
                Self::populate_symbols(&mut symbols, file, None)?;
            }
        }

        self.channel.send(request_id, Some(symbols))?;
        Ok(())
    }

    /// Handler for `textDocument/references`.
    fn text_document_references(
        &mut self,
        request_id: Option<RequestId>,
        params: ty::ReferenceParams,
    ) -> Result<()> {
        let url = params.text_document_position.text_document.uri;

        let mut locations: Vec<ty::Location> = Vec::new();

        if let Some(workspace) = &mut self.workspace {
            if let Some(references) =
                workspace.find_reference(&url, params.text_document_position.position)
            {
                for (url, ranges) in references {
                    for r in ranges {
                        locations.push(ty::Location {
                            uri: url.clone(),
                            range: convert_range(r),
                        });
                    }
                }
            }
        }

        self.channel.send(request_id, Some(locations))?;
        Ok(())
    }

    /// Handler for `workspace/didChangeConfiguration`.
    fn workspace_did_change_configuration(
        &mut self,
        _: Option<RequestId>,
        _: ty::DidChangeConfigurationParams,
    ) -> Result<()> {
        Ok(())
    }

    /// Send all diagnostics for a workspace.
    fn send_workspace_diagnostics(&self) -> Result<()> {
        if let Some(workspace) = &self.workspace {
            self.send_manifest_diagnostics(workspace)?;

            let mut by_url: HashMap<Url, Vec<(&Source, &Diagnostic)>> = HashMap::new();

            for diagnostics in &workspace.reporter {
                match *diagnostics {
                    Reported::Diagnostics(ref diagnostics) => {
                        if let Some(url) = diagnostics.source.url() {
                            let items = by_url.entry(url.clone()).or_insert_with(Vec::new);

                            for d in diagnostics.items() {
                                items.push((&diagnostics.source, d));
                            }
                        }
                    }
                    Reported::SourceDiagnostics(ref diagnostics) => {
                        for d in diagnostics.items() {
                            if let Some(url) = d.0.url() {
                                by_url
                                    .entry(url)
                                    .or_insert_with(Vec::new)
                                    .push((&d.0, &d.1));
                            }
                        }
                    }
                }
            }

            for file in workspace.files() {
                let by_url = by_url.remove(&file.url);
                let by_url_chain = by_url.into_iter().flat_map(|d| d.into_iter()).map(|d| d.1);

                self.send_diagnostics(
                    &file.url,
                    &file.diag.source,
                    file.diag.items().chain(by_url_chain),
                )?;
            }

            // diagnostics about other random files
            for (url, diag) in by_url {
                for (source, d) in diag {
                    self.send_diagnostics(&url, source, ::std::iter::once(d))?;
                }
            }
        }

        Ok(())
    }

    /// Send manifest diagnostics.
    fn send_manifest_diagnostics(&self, workspace: &Workspace) -> Result<()> {
        let mut diagnostics = Vec::new();

        if let Some(e) = workspace.manifest_error.as_ref() {
            let d = ty::Diagnostic {
                message: e.display().to_string(),
                severity: Some(ty::DiagnosticSeverity::ERROR),
                ..ty::Diagnostic::default()
            };

            diagnostics.push(d);
        }

        let url = workspace.manifest_url()?;

        self.channel
            .notification::<ty::notification::PublishDiagnostics>(ty::PublishDiagnosticsParams {
                uri: url,
                diagnostics: diagnostics,
                version: None,
            })?;

        Ok(())
    }

    /// Send diagnostics for a single URL.
    fn send_diagnostics<'a, I>(&self, url: &Url, source: &Source, diagnostics: I) -> Result<()>
    where
        I: IntoIterator<Item = &'a Diagnostic>,
    {
        let mut out = Vec::new();

        for d in diagnostics.into_iter() {
            match *d {
                reproto_core::Diagnostic::Error {
                    ref span,
                    ref message,
                } => {
                    let (start, end) = source.span_to_range(*span, Encoding::Utf16)?;
                    let range = convert_range((start, end));

                    let d = ty::Diagnostic {
                        range: range,
                        message: message.to_string(),
                        severity: Some(ty::DiagnosticSeverity::ERROR),
                        ..ty::Diagnostic::default()
                    };

                    out.push(d);
                }
                reproto_core::Diagnostic::Info {
                    ref span,
                    ref message,
                } => {
                    let (start, end) = source.span_to_range(*span, Encoding::Utf16)?;
                    let range = convert_range((start, end));

                    let d = ty::Diagnostic {
                        range: range,
                        message: message.to_string(),
                        severity: Some(ty::DiagnosticSeverity::INFORMATION),
                        ..ty::Diagnostic::default()
                    };

                    out.push(d);
                }
                _ => {}
            }
        }

        self.channel
            .notification::<ty::notification::PublishDiagnostics>(ty::PublishDiagnosticsParams {
                uri: url.clone(),
                diagnostics: out,
                version: None,
            })?;

        Ok(())
    }

    /// Handler for `textDocument/didSave`.
    fn text_document_did_save(&mut self, _: ty::DidSaveTextDocumentParams) -> Result<()> {
        if let Some(workspace) = &mut self.workspace {
            workspace.reload()?;
        }

        self.send_workspace_diagnostics()?;
        Ok(())
    }

    /// Handler for `textDocument/didChange`.
    fn text_document_did_change(&mut self, params: ty::DidChangeTextDocumentParams) -> Result<()> {
        let text_document = params.text_document;
        let url = text_document.uri;

        {
            let workspace = match &mut self.workspace {
                Some(workspace) => workspace,
                None => return Ok(()),
            };

            if params.content_changes.is_empty() {
                return Ok(());
            }

            match workspace.open_files.get_mut(&url) {
                Some(source) => {
                    let rope = match source.as_mut_rope() {
                        Some(rope) => rope,
                        None => return Ok(()),
                    };

                    apply_content_changes(rope, &params.content_changes)?;
                }
                None => return Ok(()),
            }

            workspace.dirty(&url)?;
            workspace.reload()?;
        }

        self.send_workspace_diagnostics()?;
        return Ok(());

        /// Apply a set of content changes to a rope.
        fn apply_content_changes(
            rope: &mut Rope,
            content_changes: &Vec<ty::TextDocumentContentChangeEvent>,
        ) -> Result<()> {
            for content_change in content_changes {
                let start = match content_change.range {
                    // replace range
                    Some(ref range) => {
                        // need to fetch the row, re-encode it as UTF-16 to translate it to
                        // UTF-8 ranges.
                        let start = translate_rope_position(&rope, range.start)?;
                        let end = translate_rope_position(&rope, range.end)?;

                        rope.remove(start..end);
                        start
                    }
                    // replace all
                    None => {
                        rope.remove(..);
                        0
                    }
                };

                if content_change.text != "" {
                    rope.insert(start, &content_change.text);
                }
            }

            Ok(())
        }

        /// translate the incoming position.
        fn translate_rope_position(rope: &Rope, position: ty::Position) -> Result<usize> {
            let line = rope.line(position.line as usize);

            // encoding target.
            let character = position.character as usize;

            let mut utf16_offset = 0usize;
            let mut char_offset = 0usize;

            for c in line.chars() {
                if utf16_offset == character {
                    break;
                }

                if utf16_offset > character {
                    return Err("character is not on an offset boundary".into());
                }

                utf16_offset += c.len_utf16();
                char_offset += 1;
            }

            Ok(rope.line_to_char(position.line as usize) + char_offset)
        }
    }

    /// Raise an error indicating that the current file does not belong to a manifest, or that
    /// a manifest _does not_ exist.
    fn handle_manifest_error(
        workspace: &mut Workspace,
        channel: &mut Channel<W>,
        expected: &mut HashMap<RequestId, Expected>,
    ) -> Result<()> {
        // warn if the currently opened file is not part of workspace.
        let manifest_url = workspace.manifest_url()?;

        if workspace.manifest_path.is_file() {
            let mut actions = Vec::new();

            actions.push(ty::MessageActionItem {
                title: "Ignore".to_string(),
                properties: Default::default(),
            });

            actions.push(ty::MessageActionItem {
                title: "Open project manifest".to_string(),
                properties: Default::default(),
            });

            let message = format!(
                "This file is not part of project, consider updating the [packages] section in \
                 reproto.toml"
            );

            let id = channel.request::<ty::request::ShowMessageRequest>(
                ty::ShowMessageRequestParams {
                    typ: ty::MessageType::WARNING,
                    message,
                    actions: Some(actions),
                },
            )?;

            expected.insert(id, Expected::ProjectAddMissing);
        } else {
            let mut actions = Vec::new();

            log::warn!("missing reproto manifest: {}", manifest_url);

            actions.push(ty::MessageActionItem {
                title: "Ignore".to_string(),
                properties: Default::default(),
            });

            actions.push(ty::MessageActionItem {
                title: "Initialize project".to_string(),
                properties: Default::default(),
            });

            let message = format!("Workspace does not have a reproto manifest!");

            let params = ty::ShowMessageRequestParams {
                typ: ty::MessageType::WARNING,
                message: message,
                actions: Some(actions),
            };

            let id = channel.request::<ty::request::ShowMessageRequest>(params)?;

            expected.insert(id, Expected::ProjectInit);
        }

        Ok(())
    }

    /// Handler for `textDocument/didOpen`.
    fn text_document_did_open(&mut self, params: ty::DidOpenTextDocumentParams) -> Result<()> {
        let text_document = params.text_document;
        let url = text_document.uri;
        let text = text_document.text;

        if let Some(workspace) = &mut self.workspace {
            // NOTE: access workspace.files is intentional to only access files which are not
            // already open.
            let (source, built) = match workspace.files.get(&url) {
                Some(file) => {
                    let rope = Rope::from_str(&text);

                    // NOTE: must inherit read-onlyness from the source file.
                    let source =
                        Source::rope(url.clone(), rope).with_read_only(file.diag.source.read_only);

                    (source.clone(), true)
                }
                None => {
                    let rope = Rope::from_str(&text);
                    (Source::rope(url.clone(), rope), false)
                }
            };

            if !built {
                log::debug!(
                    "url: {:?}, manifest_url: {:?}",
                    url,
                    workspace.manifest_url()?
                );

                if url != workspace.manifest_url()? {
                    Self::handle_manifest_error(workspace, &mut self.channel, &mut self.expected)?;
                }
            }

            workspace.open_files.insert(url.clone(), source);
            workspace.reload()?;
        }

        self.send_workspace_diagnostics()?;
        return Ok(());
    }

    /// Handler for `textDocument/didClose`.
    fn text_document_did_close(&mut self, params: ty::DidCloseTextDocumentParams) -> Result<()> {
        let text_document = params.text_document;

        if let Some(workspace) = &mut self.workspace {
            let url = text_document.uri;
            workspace.open_files.remove(&url);
            workspace.reload()?;
        }

        self.send_workspace_diagnostics()?;
        Ok(())
    }

    /// Handler for `textDocument/completion`.
    fn text_document_completion(
        &mut self,
        request_id: Option<RequestId>,
        params: ty::CompletionParams,
    ) -> Result<()> {
        let mut response = ty::CompletionList {
            ..ty::CompletionList::default()
        };

        self.completion_items(params, &mut response)?;
        self.channel.send(request_id, response)?;
        Ok(())
    }

    /// Populate completion items for the given request.
    fn completion_items(
        &mut self,
        params: ty::CompletionParams,
        list: &mut ty::CompletionList,
    ) -> Result<()> {
        let url = params.text_document_position.text_document.uri;

        let workspace = match &mut self.workspace {
            Some(workspace) => workspace,
            None => return Ok(()),
        };

        let (file, value) =
            match workspace.find_completion(&url, params.text_document_position.position) {
                Some(v) => v,
                None => return Ok(()),
            };

        log::debug!("type completion: {:?}", value);

        match *value {
            Completion::Package { ref results, .. } => {
                for r in results {
                    list.items.push(ty::CompletionItem {
                        label: r.to_string(),
                        kind: Some(ty::CompletionItemKind::MODULE),
                        ..ty::CompletionItem::default()
                    });
                }
            }
            Completion::Any { ref suffix } => {
                for (prefix, value) in &file.prefixes {
                    list.items.push(ty::CompletionItem {
                        label: format!("{}::", prefix),
                        kind: Some(ty::CompletionItemKind::MODULE),
                        detail: Some(value.package.to_string()),
                        ..ty::CompletionItem::default()
                    });
                }

                for c in &self.built_ins {
                    list.items.push(ty::CompletionItem {
                        label: c.to_string(),
                        kind: Some(ty::CompletionItemKind::KEYWORD),
                        ..ty::CompletionItem::default()
                    });
                }

                let path = vec![];
                push_items(&mut list.items, &file, &path, suffix)?;
            }
            Completion::Absolute {
                ref prefix,
                ref path,
                ref suffix,
            } => {
                let file = if let Some(ref prefix) = *prefix {
                    match file
                        .prefixes
                        .get(prefix)
                        .and_then(|p| workspace.packages.get(&p.package))
                        .and_then(|url| workspace.file(url))
                    {
                        Some(file) => file,
                        None => return Ok(()),
                    }
                } else {
                    file
                };

                push_items(&mut list.items, &file, path, suffix)?;
            }
        }

        return Ok(());

        fn push_items(
            items: &mut Vec<ty::CompletionItem>,
            file: &LoadedFile,
            path: &Vec<String>,
            suffix: &Option<String>,
        ) -> Result<()> {
            // access all symbols for exact matching symbol.
            if let Some(ref suffix) = *suffix {
                let mut path = path.clone();
                path.push(suffix.to_string());

                if let Some(symbols) = file.symbols.get(&path) {
                    for s in symbols {
                        items.push(ty::CompletionItem {
                            label: format!("{}::{}", suffix.to_string(), s.name),
                            kind: Some(ty::CompletionItemKind::CLASS),
                            documentation: s.to_documentation(),
                            ..ty::CompletionItem::default()
                        });
                    }
                };
            }

            if let Some(symbols) = file.symbols.get(path) {
                for s in symbols {
                    items.push(ty::CompletionItem {
                        label: s.name.to_string(),
                        kind: Some(ty::CompletionItemKind::CLASS),
                        documentation: s.to_documentation(),
                        ..ty::CompletionItem::default()
                    });
                }
            };

            Ok(())
        }
    }

    fn completion_item_resolve(&mut self, _: ty::CompletionItem) -> Result<()> {
        Ok(())
    }

    /// Handler for `textDocument/definition`.
    fn text_document_definition(
        &mut self,
        request_id: Option<RequestId>,
        params: ty::TextDocumentPositionParams,
    ) -> Result<()> {
        let mut response = None::<ty::GotoDefinitionResponse>;
        self.definition(params, &mut response)?;
        self.channel.send(request_id, response)?;
        Ok(())
    }

    /// Handler for renaming
    fn text_document_rename(
        &mut self,
        request_id: Option<RequestId>,
        params: ty::RenameParams,
    ) -> Result<()> {
        let workspace = match &mut self.workspace {
            Some(workspace) => workspace,
            None => return Err("no workspace".into()),
        };

        let url = params.text_document_position.text_document.uri;
        let new_name = params.new_name;

        let mut edit: Option<ty::WorkspaceEdit> = None;

        if let Some(rename) = workspace.find_rename(&url, params.text_document_position.position) {
            match rename {
                // all edits in the same file as where the rename was requested.
                RenameResult::Local { ranges } => {
                    let edits = setup_edits(ranges, new_name.as_str());
                    edit = Some(local_edits(&url, edits));
                }
                // A collection of ranges from different URLs that should be changed.
                RenameResult::Collections { ranges } => {
                    let mut changes = Vec::new();

                    for (url, ranges) in ranges {
                        let edits = setup_edits(ranges, new_name.as_str());

                        changes.push(ty::TextDocumentEdit {
                            text_document: ty::OptionalVersionedTextDocumentIdentifier {
                                uri: url.clone(),
                                version: None,
                            },
                            edits,
                        });
                    }

                    edit = Some(ty::WorkspaceEdit {
                        document_changes: Some(ty::DocumentChanges::Edits(changes)),
                        ..ty::WorkspaceEdit::default()
                    });
                }
                // Special case: identical to Local, but we also want to refactor the package
                // position to include the new alias.
                RenameResult::ImplicitPackage { ranges, position } => {
                    let mut edits = setup_edits(ranges, new_name.as_str());

                    edits.push(ty::OneOf::Left(ty::TextEdit {
                        range: convert_range((position, position)),
                        new_text: format!(" as {}", new_name),
                    }));

                    edit = Some(local_edits(&url, edits));
                }
                RenameResult::NotSupported => {
                    log::info!("not supported");
                }
            };
        }

        self.channel.send(request_id, edit)?;
        return Ok(());

        fn setup_edits(
            ranges: &Vec<Range>,
            new_text: &str,
        ) -> Vec<ty::OneOf<ty::TextEdit, ty::AnnotatedTextEdit>> {
            let mut edits = Vec::new();

            for range in ranges {
                edits.push(ty::OneOf::Left(ty::TextEdit {
                    range: convert_range(range),
                    new_text: new_text.to_string(),
                }));
            }

            edits
        }

        // Setup a workspace edit which is only local to the specified URL.
        fn local_edits(
            url: &Url,
            edits: Vec<ty::OneOf<ty::TextEdit, ty::AnnotatedTextEdit>>,
        ) -> ty::WorkspaceEdit {
            let changes = vec![ty::TextDocumentEdit {
                text_document: ty::OptionalVersionedTextDocumentIdentifier {
                    uri: url.clone(),
                    version: None,
                },
                edits: edits,
            }];

            ty::WorkspaceEdit {
                document_changes: Some(ty::DocumentChanges::Edits(changes)),
                ..ty::WorkspaceEdit::default()
            }
        }
    }

    /// Populate the goto definition response.
    fn definition(
        &mut self,
        params: ty::TextDocumentPositionParams,
        response: &mut Option<ty::GotoDefinitionResponse>,
    ) -> Result<()> {
        let url = params.text_document.uri;

        let workspace = match &mut self.workspace {
            Some(workspace) => workspace,
            None => return Ok(()),
        };

        let (file, value) = match workspace.find_jump(&url, params.position) {
            Some(v) => v,
            None => return Ok(()),
        };

        log::debug!("jump: {}: {:?}", file.url, value);

        match *value {
            Jump::Absolute {
                ref package,
                ref path,
            } => {
                let (uri, file) = if let Some(ref package) = *package {
                    let url = match workspace.packages.get(package) {
                        Some(url) => url,
                        None => return Ok(()),
                    };

                    match workspace.file(url) {
                        Some(file) => (url.clone(), file),
                        None => return Ok(()),
                    }
                } else {
                    (url, file)
                };

                let span = match file.symbol.get(path) {
                    Some(span) => *span,
                    None => return Ok(()),
                };

                let (start, end) = file.diag.source.span_to_range(span, Encoding::Utf16)?;
                let range = convert_range((start, end));
                let location = ty::Location { uri, range };

                *response = Some(ty::GotoDefinitionResponse::Scalar(location));
            }
            Jump::Package { ref package } => {
                let uri = match workspace.packages.get(package) {
                    Some(url) => url.clone(),
                    None => return Ok(()),
                };

                let range = ty::Range::default();
                let location = ty::Location { uri, range };
                *response = Some(ty::GotoDefinitionResponse::Scalar(location));
            }
            Jump::Prefix { ref prefix } => {
                let prefix = match file.prefixes.get(prefix) {
                    Some(prefix) => prefix,
                    None => return Ok(()),
                };

                let range = convert_range(prefix.range);

                let location = ty::Location {
                    uri: url.clone(),
                    range,
                };

                *response = Some(ty::GotoDefinitionResponse::Scalar(location));
            }
        }

        Ok(())
    }
}

/// Convert an internal range into a language-server range.
fn convert_range<R: Into<Range>>(range: R) -> ty::Range {
    let range = range.into();

    let start = range.start;
    let end = range.end;

    let start = ty::Position {
        line: start.line as u32,
        character: start.col as u32,
    };

    let end = ty::Position {
        line: end.line as u32,
        character: end.col as u32,
    };

    ty::Range { start, end }
}

#[derive(Debug, Clone)]
pub enum Expected {
    /// Feedback from project init.
    ProjectInit,
    /// Feedback from project add missing.
    ProjectAddMissing,
}

/// $/openUrl custom notification.
pub enum OpenUrl {}

impl ty::notification::Notification for OpenUrl {
    type Params = Url;

    const METHOD: &'static str = "$/openUrl";
}
