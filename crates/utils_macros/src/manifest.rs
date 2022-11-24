use proc_macro::TokenStream;
use std::{env, path::PathBuf};
use toml::map::Map;

/// Manifest for `Cargo.toml` file.
pub struct Manifest {
    manifest: Map<String, toml::Value>,
}

impl Manifest {
    /// Returns a [`syn::Path`] for the `crate` with `name`.
    pub fn get_path(name: &str) -> syn::Path {
        Self::default()
            .maybe_get_path(name)
            .unwrap_or_else(|| Self::parse_str(name))
    }

    fn parse_str<T: syn::parse::Parse>(path: &str) -> T {
        syn::parse(path.parse::<TokenStream>().unwrap()).unwrap()
    }

    fn maybe_get_path(&self, name: &str) -> Option<syn::Path> {
        const QINETIC: &str = "qinetic";
        const QINETIC_INTERNAL: &str = "qinetic_internal";

        fn dep_package(dep: &toml::Value) -> Option<&str> {
            if dep.as_str().is_some() {
                None
            } else {
                dep.as_table()
                    .unwrap()
                    .get("package")
                    .map(|name| name.as_str().unwrap())
            }
        }

        let find_in_deps = |deps: &Map<String, toml::Value>| -> Option<syn::Path> {
            let package = if let Some(dep) = deps.get(name) {
                return Some(Self::parse_str(dep_package(dep).unwrap_or(name)));
            } else if let Some(dep) = deps.get(QINETIC) {
                dep_package(dep).unwrap_or(QINETIC)
            } else if let Some(dep) = deps.get(QINETIC_INTERNAL) {
                dep_package(dep).unwrap_or(QINETIC_INTERNAL)
            } else {
                return None;
            };

            let mut path = Self::parse_str::<syn::Path>(package);
            if let Some(module) = name.strip_prefix("qinetic_") {
                path.segments.push(Self::parse_str(module));
            }
            Some(path)
        };

        let deps = self
            .manifest
            .get("dependencies")
            .map(|deps| deps.as_table().unwrap());
        let deps_dev = self
            .manifest
            .get("dev-dependencies")
            .map(|deps| deps.as_table().unwrap());

        deps.and_then(find_in_deps)
            .or_else(|| deps_dev.and_then(find_in_deps))
    }
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            manifest: env::var_os("CARGO_MANIFEST_DIR")
                .map(PathBuf::from)
                .map(|mut path| {
                    path.push("Cargo.toml");
                    let manifest = std::fs::read_to_string(path).unwrap();
                    toml::from_str(&manifest).unwrap()
                })
                .unwrap(),
        }
    }
}
