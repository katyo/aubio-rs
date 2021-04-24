extern crate pkg_config;

use std::env;

#[cfg(feature = "generate-bindings")]
mod source {
    pub const URL: &str = "https://github.com/katyo/{package}-rs/releases/download/{package}-{version}/{package}-{version}.tar.gz";
    pub const VERSION: &str = "0.5.0-git1f23a23d";
}

fn main() {
    let build = env::var("CARGO_FEATURE_BUILD").is_ok();
    if build {
        println!("cargo:rustc-link-lib=aubio");
    } else {
        if let Ok(paths) = pkg_config::Config::new()
            .atleast_version("0.4.9")
            .probe("aubio")
        {
            for path in paths.include_paths {
                println!("{}", path.display());
            }
        }
    }

    #[cfg(feature = "generate-bindings")]
    {
        use std::{env, path::Path};

        let src = utils::Source::new(
            "aubio",
            env::var("AUBIO_VERSION").unwrap_or(source::VERSION.into()),
            env::var("AUBIO_URL").unwrap_or(source::URL.into()),
        );

        let out_dir = env::var("OUT_DIR").expect("The OUT_DIR is set by cargo.");

        let out_dir = Path::new(&out_dir);

        let src_dir = out_dir.join("source").join(&src.version);

        utils::fetch_source(&src, &src_dir);

        let inc_dir = src_dir.join("src");
        let bindings = out_dir.join("bindings.rs");

        utils::generate_bindings(&inc_dir, &bindings);
    }
}

#[cfg(feature = "generate-bindings")]
mod utils {
    use std::path::Path;

    pub struct Source {
        pub package: String,
        pub version: String,
        pub url: String,
    }

    impl Source {
        pub fn new(
            package: impl Into<String>,
            version: impl Into<String>,
            url: impl Into<String>,
        ) -> Self {
            Self {
                package: package.into(),
                version: version.into(),
                url: url.into(),
            }
        }

        pub fn url(&self) -> String {
            self.url
                .replace("{package}", &self.package)
                .replace("{version}", &self.version)
        }
    }

    pub fn fetch_source(src: &Source, out_dir: &Path) {
        use fetch_unroll::Fetch;

        if !out_dir.is_dir() {
            let src_url = src.url();

            eprintln!(
                "Fetch aubio sources from {} to {}",
                src_url,
                out_dir.display()
            );

            Fetch::from(src_url)
                .unroll()
                .strip_components(1)
                .to(out_dir)
                .expect("Aubio sources should be fetched");
        }
    }

    pub fn generate_bindings(inc_dir: &Path, out_file: &Path) {
        let bindings = bindgen::Builder::default()
            .detect_include_paths(true)
            .clang_args(&[format!("-I{}", inc_dir.display())])
            .header(inc_dir.join("aubio.h").display().to_string())
            .generate()
            .expect("Generated bindings");

        bindings.write_to_file(out_file).expect("Writen bindings");
    }
}
