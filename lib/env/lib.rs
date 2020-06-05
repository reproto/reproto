mod config;
mod config_env;
mod initialize;

pub use self::config_env::ConfigEnvironment;
pub use self::initialize::initialize;
use core::errors::Result;
use core::{RelativePath, Resolver};
use manifest::{Lang, Language, Manifest};
use repository::{
    index_from_path, index_from_url, objects_from_path, objects_from_url, Index, IndexConfig,
    NoIndex, NoObjects, Objects, ObjectsConfig, Paths, Repository, Resolvers,
};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

pub const DEFAULT_INDEX: &'static str = "git+https://github.com/reproto/reproto-index";
pub const MANIFEST_NAME: &'static str = "reproto.toml";

fn load_index(
    base: &Path,
    url: &str,
    publishing: bool,
    config: IndexConfig,
) -> Result<Box<dyn Index>> {
    let index_path = Path::new(url);

    if index_path.is_dir() {
        let index_path = index_path
            .canonicalize()
            .map_err(|e| format!("index: bad path: {}: {}", e, index_path.display()))?;

        return index_from_path(&index_path).map_err(Into::into);
    }

    match url::Url::parse(url) {
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            let path = RelativePath::new(url).to_path(base);

            index_from_path(&path).map_err(Into::into)
        }
        Err(e) => return Err(e.into()),
        Ok(url) => index_from_url(config, &url, publishing).map_err(Into::into),
    }
}

fn load_objects(
    index: &dyn Index,
    index_publishing: bool,
    index_url: &str,
    objects: Option<String>,
    config: ObjectsConfig,
) -> Result<Box<dyn Objects>> {
    let (objects_url, publishing) = if let Some(ref objects) = objects {
        (objects.as_ref(), true)
    } else {
        (index.objects_url()?, index_publishing)
    };

    log::debug!("index: {}", index_url);
    log::debug!("objects: {}", objects_url);

    let objects_path = Path::new(objects_url);

    if objects_path.is_dir() {
        let objects_path = objects_path
            .canonicalize()
            .map_err(|e| format!("objects: bad path: {}: {}", e, objects_path.display()))?;

        return objects_from_path(objects_path);
    }

    match url::Url::parse(objects_url) {
        // Relative to index index repository!
        Err(url::ParseError::RelativeUrlWithoutBase) => index
            .objects_from_index(RelativePath::new(objects_url))
            .map_err(Into::into),
        Err(e) => return Err(e.into()),
        Ok(url) => objects_from_url(
            config,
            &url,
            |config, scheme, url| match scheme {
                "http" => Ok(Some(repository_http::objects_from_url(config, url)?)),
                "https" => Ok(Some(repository_http::objects_from_url(config, url)?)),
                _ => Ok(None),
            },
            publishing,
        )
        .map_err(Into::into),
    }
}

pub fn repository(manifest: &Manifest) -> Result<Repository> {
    let repository = &manifest.repository;

    if repository.no_repository {
        return Ok(Repository::new(Box::new(NoIndex), Box::new(NoObjects)));
    }

    let base = manifest
        .path
        .as_ref()
        .ok_or_else(|| format!("manifest does not have a path"))
        .and_then(|p| {
            p.parent()
                .ok_or_else(|| format!("no parent path to manifest: {}", p.display()))
        })?;

    let mut repo_dir = None;
    let mut cache_home = None;
    let mut index = repository.index.clone();
    let mut objects = repository.objects.clone();

    if let Some(config_env) = ConfigEnvironment::new()? {
        repo_dir = Some(config_env.repo_dir);
        cache_home = Some(config_env.cache_home);
        index = index.or(config_env.index.clone());
        objects = objects.or(config_env.objects.clone());
    }

    let repo_dir = repo_dir.ok_or_else(|| "repo_dir: must be specified")?;

    // NB: do not permit publishing to default index.
    let (index_url, index_publishing) = index
        .map(|index| (index, true))
        .unwrap_or_else(|| (DEFAULT_INDEX.to_owned(), false));

    let index_config = IndexConfig {
        repo_dir: repo_dir.clone(),
    };

    let index = load_index(base, index_url.as_str(), index_publishing, index_config)?;

    let objects_config = ObjectsConfig {
        repo_dir,
        cache_home,
        missing_cache_time: Some(Duration::new(60, 0)),
    };

    let objects = load_objects(
        index.as_ref(),
        index_publishing,
        index_url.as_str(),
        objects,
        objects_config,
    )?;

    Ok(Repository::new(index, objects))
}

/// Setup the path-based resolver from a manifest.
pub fn path_resolver(manifest: &Manifest) -> Result<Option<Box<dyn Resolver>>> {
    if manifest.paths.is_empty() {
        return Ok(None);
    }

    let mut published = HashMap::new();

    if let Some(publish) = manifest.publish.as_ref() {
        for p in publish {
            published.insert(p.package.clone(), p.version.clone());
        }
    }

    Ok(Some(Box::new(Paths::new(
        manifest.paths.clone(),
        published,
    ))))
}

/// Set up the all resolvers based on this manifest.
pub fn resolver(manifest: &manifest::Manifest) -> Result<Box<dyn Resolver>> {
    resolver_with_extra(manifest, None)
}

/// Resolver with an extra resolver prepended to it.
pub fn resolver_with_extra(
    manifest: &manifest::Manifest,
    extra: Option<Box<dyn Resolver>>,
) -> Result<Box<dyn Resolver>> {
    let mut resolvers = Vec::<Box<dyn Resolver>>::new();

    resolvers.extend(extra);
    resolvers.extend(path_resolver(manifest)?);
    resolvers.push(Box::new(repository(manifest)?));

    Ok(Box::new(Resolvers::new(resolvers)))
}

/// Convert the manifest language to an actual language implementation.
pub fn convert_lang(input: Language) -> Box<dyn Lang> {
    use self::Language::*;

    match input {
        Csharp => Box::new(csharp::CsharpLang),
        Dart => Box::new(dart::DartLang),
        Go => Box::new(go::GoLang),
        Java => Box::new(java::JavaLang),
        Js => Box::new(js::JsLang),
        Json => Box::new(json::JsonLang),
        Python => Box::new(python::PythonLang),
        Reproto => Box::new(reproto::ReprotoLang),
        Rust => Box::new(rust::RustLang),
        Swift => Box::new(swift::SwiftLang),
        OpenApi => Box::new(openapi::OpenApiLang),
    }
}
