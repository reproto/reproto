import * as React from "react";
import {Input} from "./Input";
import {OutputEditor} from "./OutputEditor";
import {JavaSettings, JavaSettingsForm} from "./JavaSettings";
import {CsharpSettings, CsharpSettingsForm} from "./CsharpSettings";
import {RustSettings, RustSettingsForm} from "./RustSettings";
import * as WebAssembly from "webassembly";
import {Annotation, Marker as AceMarker} from 'react-ace';
import AceEditor from 'react-ace';

const wasm = require("rust/reproto-wasm.js");

const deepEqual = require("deep-equal");

const languages = [
  "yaml",
  "json",
  "java",
  "csharp",
  "rust",
  "python",
  "javascript",
]

const themes = [
  "monokai",
  "github",
]

const FORMAT_LANGUAGE_MAP: {[key: string]: string} = {
  yaml: "yaml",
  json: "json",
  java: "java",
  csharp: "csharp",
  rust: "rust",
  python: "python",
  js: "javascript",
  reproto: "reproto",
};

// modes in local_modules
require("brace/mode/reproto.js")

languages.forEach((lang) => {
  require(`brace/mode/${lang}`)
  require(`brace/snippets/${lang}`)
})

themes.forEach((theme) => {
  require(`brace/theme/${theme}`)
})

const DEFAULT_JSON = require("raw-loader!../static/default.json");
const DEFAULT_YAML = require("raw-loader!../static/default.yaml");
const COMMON_REPROTO: string = require("raw-loader!../static/common.reproto");
const IMPORT_REPROTO: string = require("raw-loader!../static/import.reproto");
const TYPE_REPROTO: string = require("raw-loader!../static/type.reproto");
const INTERFACE_REPROTO: string = require("raw-loader!../static/interface.reproto");
const DEFAULT_NEW_FILE_REPROTO: string = require("raw-loader!../static/default-new.reproto");
const logo = require("../static/logo.256.png");

interface Compiled {
  request: Derive;
  result: DeriveResult;
}

interface Derive {
  content: any;
  root_name: string;
  format: string;
  output: string;
  package_prefix: string;
}

interface Marker {
  message: string;
  row_start: number;
  row_end: number;
  col_start: number;
  col_end: number;
}

interface DeriveResult {
  result?: string;
  error?: string;
  error_markers: Marker[];
  info_markers: Marker[];
}

enum Format {
  Json = "json",
  Yaml = "yaml",
  Reproto = "reproto",
}

enum Output {
  Reproto = "reproto",
  Java = "java",
  Csharp = "csharp",
  Rust = "rust",
  Python = "python",
  JavaScript = "js",
  Json = "json",
}

export interface MainProps {
}

interface ContentSet {
  [key: string]: string;
}

interface File {
  package: string;
  version?: string;
  content: string;
}

interface Settings {
  java: JavaSettings;
  csharp: CsharpSettings;
  rust: RustSettings;
}

export interface MainState {
  contentSet: ContentSet;
  // set of files
  files: File[],
  // current selected package.
  file_index: number,
  // If we are editing the file metadata right now.
  file_editing_meta: boolean;
  // Settings for various outputs.
  settings: Settings;
  format: Format;
  output: Output;
  root_name: string;
  package_prefix: string;
  // Error annotations (gutter markers) on input.
  input_annotations: Annotation[];
  // Error markers on input.
  input_markers: AceMarker[];
  // Result of last compilation.
  compiled?: Compiled;
  error?: string;
  derive?: (value: Derive) => DeriveResult;
}

export class Main extends React.Component<MainProps, MainState> {
  constructor(props: MainProps) {
    super(props);

    this.state = {
      contentSet: {
        json: DEFAULT_JSON,
        yaml: DEFAULT_YAML,
      },
      files: [
        {
          package: "example.type",
          content: TYPE_REPROTO,
        },
        {
          package: "example.interface",
          content: INTERFACE_REPROTO,
        },
        {
          package: "example.common",
          version: "1.0.0",
          content: COMMON_REPROTO,
        },
        {
          package: "example.import",
          content: IMPORT_REPROTO,
        },
      ],
      file_index: 0,
      file_editing_meta: false,
      settings: {
        java: {
          jackson: true,
          lombok: true,
        },
        rust: {
          chrono: true,
        },
        csharp: {
          json_net: true,
        }
      },
      root_name: "Generated",
      package_prefix: "reproto",
      input_annotations: [],
      input_markers: [],
      format: Format.Reproto,
      output: Output.Java,
    };
  }

  componentDidMount() {
    fetch("reproto-wasm.wasm")
      .then(response => response.arrayBuffer())
      .then(buffer => WebAssembly.compile(buffer))
      .then(mod => wasm(mod, true))
      .then(mod => {
      this.setState({derive: mod.derive}, () => this.recompile());
    });
  }

  content(): string {
    let {format} = this.state;

    if (format == "reproto") {
      return this.state.files[this.state.file_index].content;
    } else {
      return this.state.contentSet[format];
    }
  }

  recompile() {
    this.setState((state: MainState, props: MainProps) => {
      let {
        contentSet,
        format,
        output,
        root_name,
        package_prefix,
        files,
        file_index,
        settings,
        compiled,
        derive,
      } = state;

      let content = this.content();

      if (!derive) {
        return {};
      }

      let compile = true;

      let content_request;

      if (format == "reproto") {
        content_request = {type: "file_index", index: file_index};
      } else {
        content_request = {type: "content", content: content};
      }

      const request = {
        content: content_request,
        files: files,
        root_name: root_name,
        package_prefix: package_prefix,
        format: format,
        output: output,
        settings: settings,
      };

      // no need to dispatch new request if it's identical to the old one...
      if (compiled && deepEqual(compiled.request, request)) {
        return {} as MainProps;
      }

      const result = derive(request) as DeriveResult;

      console.log("derive", request, result);

      const input_annotations: Annotation[] = [];
      const input_markers: AceMarker[] = [];

      result.error_markers.forEach(m => {
        input_annotations.push({
          row: m.row_start,
          column: m.col_start,
          type: 'error',
          text: m.message,
        });

        input_markers.push({
          startRow: m.row_start,
          startCol: m.col_start,
          endRow: m.row_end,
          endCol: m.col_end,
          className: "error-marker",
          type: "background",
        });
      });

      result.info_markers.forEach(m => {
        input_annotations.push({
          row: m.row_start,
          column: m.col_start,
          type: 'info',
          text: m.message,
        });

        input_markers.push({
          startRow: m.row_start,
          startCol: m.col_start,
          endRow: m.row_end,
          endCol: m.col_end,
          className: "info-marker",
          type: "background",
        });
      });

      // Don't hide old result on errors.
      if (!result.result && compiled && compiled.result.result) {
        result.result = compiled.result.result;
      }

      return {
        compiled: {
          request: request,
          result: result,
        },
        input_annotations: input_annotations,
        input_markers: input_markers,
      };
    });
  }

  setContent(format: Format, value: string) {
    console.log("new content", value.length);

    this.setState((state: MainState, props: MainProps) => {
      let {file_index, files, contentSet} = this.state;

      if (format == "reproto") {
        let new_files = files.map((file, index) => {
          if (index == file_index) {
            let new_file = {...file};
            new_file.content = value;
            return new_file;
          } else {
            return file;
          }
        });

        return {files: new_files} as MainState;
      } else {
        let new_content_set = {...contentSet};
        new_content_set[format] = value;
        return {contentSet: new_content_set} as MainState;
      }
    }, () => this.recompile());
  }

  setFile(file_index: number, cb: (file: File) => void) {
    this.setState((state: MainState, props: MainProps) => {
      let {files} = this.state;

      let new_files = files.map((file, index) => {
        if (index == file_index) {
          let new_file = {...file};
          cb(new_file);
          return new_file;
        } else {
          return file;
        }
      });

      return {files: new_files};
    }, () => this.recompile());
  }

  setFileIndex(value: string) {
    this.setState({
      file_index: Number(value)
    }, () => this.recompile());
  }

  setFormat(value: string) {
    let format;

    switch (value) {
      case "yaml":
        format = "yaml" as Format;
        break;
      case "reproto":
        format = "reproto" as Format;
        break;
      case "json":
        format = "json" as Format;
        break;
      default:
        return;
    }

    this.setState({
      format: format
    }, () => this.recompile());
  }

  setOutput(value: string) {
    let output;

    switch (value) {
      case "reproto":
        output = "reproto" as Output;
        break;
      case "java":
        output = "java" as Output;
        break;
      case "csharp":
        output = "csharp" as Output;
        break;
      case "python":
        output = "python" as Output;
        break;
      case "rust":
        output = "rust" as Output;
        break;
      case "js":
        output = "js" as Output;
        break;
      case "json":
        output = "json" as Output;
        break;
      default:
        return;
    }

    this.setState({
      output: output
    }, () => this.recompile());
  }

  setRootName(root_name: string) {
    this.setState({
      root_name: root_name
    }, () => this.recompile());
  }

  setPackagePrefix(package_prefix: string) {
    this.setState({
      package_prefix: package_prefix
    }, () => this.recompile());
  }

  updateJava(cb: (settings: JavaSettings) => void) {
    this.setState((state: MainState, props: MainProps) => {
      let settings = {...state.settings};
      settings.java = {...settings.java};
      cb(settings.java);
      return {settings: settings};
    }, () => this.recompile());
  }

  updateRust(cb: (settings: RustSettings) => void) {
    this.setState((state: MainState, props: MainProps) => {
      let settings = {...state.settings};
      settings.rust = {...settings.rust};
      cb(settings.rust);
      return {settings: settings};
    }, () => this.recompile());
  }

  updateCsharp(cb: (settings: CsharpSettings) => void) {
    this.setState((state: MainState, props: MainProps) => {
      let settings = {...state.settings};
      settings.csharp = {...settings.csharp};
      cb(settings.csharp);
      return {settings: settings};
    }, () => this.recompile());
  }

  newFile() {
    this.setState((state: MainState, props: MainProps) => {
      let { files } = state;

      files = [...files];
      let file_index = files.length;

      files.push({
        content: DEFAULT_NEW_FILE_REPROTO,
        package: "new"
      });

      return {
        files: files,
        file_index: file_index,
        file_editing_meta: true,
      };
    }, () => this.recompile());
  }

  deleteFile() {
    this.setState((state: MainState, props: MainProps) => {
      let { files, file_index } = state;

      return {
        files: files.filter((_, i: number) => i != file_index),
        file_index: 0,
      };
    }, () => this.recompile());
  }

  render() {
    let {
      contentSet,
      files,
      file_index,
      format,
      output,
      root_name,
      package_prefix,
      input_annotations,
      input_markers,
      settings,
      compiled,
      derive,
    } = this.state;

    let content = this.content();

    let input_mode = FORMAT_LANGUAGE_MAP[format as string];
    let output_mode = FORMAT_LANGUAGE_MAP[output as string];

    let errorMessage;
    let compiledResult;

    var wasmLoading;
    var settingsForm;

    var packages;

    if (format == "reproto") {
      let {version, package: file_package} = files[file_index];
      let {file_editing_meta} = this.state;

      let view;

      if (file_editing_meta) {
        view = (
          <div className="form-row">
          <div className="col-auto">
            <input
              id="file-package"
              type="text"
              className="form-control form-control-sm"
              placeholder="package"
              onChange={e => {
                let value = e.target.value;
                this.setFile(file_index, file => file.package = value);
              }}
              value={file_package} />
          </div>

          <div className="col-auto">
            <input
              id="file-version"
              type="text"
              className="form-control form-control-sm"
              placeholder="version"
              onChange={e => {
                let value = e.target.value;
                this.setFile(file_index, file => {
                  if (value == "") {
                    delete file.version;
                  } else {
                    file.version = value;
                  }
                });
              }}
              value={version || ""} />
          </div>

          <div className="col-auto">
            <button type="button" className="btn btn-primary btn-sm" onClick={() => {
              this.setState({file_editing_meta: false});
            }}>
              ok
            </button>
          </div>
          </div>
        );
      } else {
        view = (
          <div className="form-row">
          <div className="col-auto">
            <select
              className="custom-select custom-select-sm"
              value={file_index}
              onChange={e => this.setFileIndex(e.target.value)}>
              { files.map((f, index) => {
                return <option key={index} value={index}>{f.package} {f.version || ""}</option>;
              }) }
            </select>
          </div>

          <div className="col-auto">
            <button type="button" className="btn btn-default btn-sm" onClick={() => {
              this.setState({file_editing_meta: true});
            }}>
              edit
            </button>
          </div>

          <div className="col-auto">
            <button type="button" className="btn btn-default btn-sm" onClick={() => {
              this.newFile();
            }}>
              new
            </button>
          </div>

          <div className="col-auto">
            <button type="button" className="btn btn-danger btn-sm" onClick={() => {
              this.deleteFile();
            }}>
              delete
            </button>
          </div>
          </div>
        );
      }

      packages = (
        <div className="row">
          <div className="col">
            <form>{view}</form>
          </div>
        </div>
      );
    }

    if (!derive) {
      wasmLoading = (
        <div className="modal" role="dialog" style={{display: "block"}}>
          <div className="modal-dialog" role="document">
            <div className="modal-content">
              <div className="modal-body">
                <div style={{textAlign: "center"}}>
                  <div className="col-auto">
                    <i className="fa fa-spinner fa-spin" style={{fontSize: "24px"}} /><br />
                  </div>
                  <br />
                  <p>Loading WASM component...</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      );
    }

    if (format) {
      switch (output) {
        case "java":
          settingsForm = <JavaSettingsForm settings={settings.java}
            onJackson={update => this.updateJava(java => java.jackson = update)}
            onLombok={update => this.updateJava(java => java.lombok = update)}
            />;
          break;
        case "rust":
          settingsForm = <RustSettingsForm settings={settings.rust}
            onChrono={update => this.updateRust(rust => rust.chrono = update)}
            />;
          break;
        case "csharp":
          settingsForm = <CsharpSettingsForm settings={settings.csharp}
            onJsonNet={update => this.updateCsharp(csharp => csharp.json_net = update)}
            />;
          break;
        default:
          break;
      }
    }

    if (compiled) {
      let { error, error_markers, result } = compiled.result;

      if (result) {
        compiledResult = result;
      }

      if (error) {
        errorMessage = (
          <div className="error row mt-2">
            <div className="col">
              {error_markers.length == 0 ?
                  <div className="alert alert-danger">{error}</div>
              : undefined }
              {error_markers.map((m, key) => {
                return (
                  <div key={key} className="alert alert-danger">
                    {m.row_start + 1}:{m.col_start}: {m.message}
                  </div>
                );
              })}
            </div>
          </div>
        );
      }
    }

    return (
      <div className="box">
        {wasmLoading}

        <div className="box-row header">
          <nav className="navbar navbar-expand-lg navbar-light bg-light">
            <a className="navbar-brand" href="https://github.com/reproto">
              <img src={logo} width={48} height={48} title="reproto" />
            </a>
            <a className="navbar-brand" href="#">reproto eval</a>
          </nav>

          <div className="container-fluid">
            <div className="row mb-2 mt-2">
              <div className="col">
                <form>
                  <div className="form-row">
                    <div className="col-auto">
                      <label htmlFor="format" className="lb-sm">Format:</label>

                      <select
                        id="format"
                        className="custom-select custom-select-sm"
                        value={format}
                        onChange={e => this.setFormat(e.target.value)}>
                        <option value="reproto">Reproto</option>
                        <option value="json">JSON (Derive)</option>
                        <option value="yaml">YAML (Derive)</option>
                      </select>
                    </div>

                    {format != "reproto" ?
                    <div className="col-auto">
                      <label htmlFor="root-name" className="lb-sm">Generated Name:</label>

                      <input
                        id="root-name"
                        type="text"
                        className="form-control form-control-sm"
                        value={root_name}
                        onChange={e => this.setRootName(e.target.value)} />
                    </div> : undefined}

                    <div className="col-auto">
                      <label htmlFor="package-prefix" className="lb-sm">Package Prefix:</label>

                      <input
                        id="package-prefix"
                        type="text"
                        className="form-control form-control-sm"
                        value={package_prefix}
                        onChange={e => this.setPackagePrefix(e.target.value)} />
                    </div>

                    <div className="col-auto">
                      <label htmlFor="output" className="lb-sm">Output:</label>

                      <select
                        id="output"
                        className="custom-select custom-select-sm"
                        value={output}
                        onChange={e => this.setOutput(e.target.value)}>
                        <option value="reproto">Reproto</option>
                        <option value="java">Java</option>
                        <option value="csharp">C#</option>
                        <option value="python">Python</option>
                        <option value="js">JavaScript</option>
                        <option value="rust">Rust</option>
                        <option value="json">JSON (RpIR)</option>
                      </select>
                    </div>

                    {settingsForm ? (
                    <div className="col-auto">
                      <label className="lb-sm">Settings:</label>
                      <div>{settingsForm}</div>
                    </div>
                    ) : undefined}
                  </div>
                </form>
              </div>
            </div>

            {packages ? (
            <div className="row mb-2 mt-2">
              <div className="col">{packages}</div>
            </div>
            ) : undefined}
          </div>
        </div>

        <div className="box-row content">
          <div className="row editors">
            <div className="col-6 col-xl-5 input">
              <AceEditor
                tabSize={2}
                showGutter={true}
                mode={input_mode}
                theme="monokai"
                width="100%"
                height="100%"
                value={content}
                annotations={input_annotations}
                markers={input_markers}
                onChange={value => this.setContent(format, value)}
                />
            </div>

            <div className="col">
              {errorMessage}

              <OutputEditor
                mode={output_mode as string}
                value={compiledResult} />
            </div>
          </div>
        </div>
      </div>
    );
  }
}
