#[cfg(feature = "generate-bindings")]
mod source {
    pub const REPOSITORY: &str = "https://aubio.org/pub/aubio-";
    pub const VERSION: &str = "0.4.9";
}

fn main() {
    #[cfg(feature = "generate-bindings")]
    {
        use std::{
            env,
            path::Path,
        };

        let src = utils::Source {
            repository: env::var("AUBIO_REPOSITORY")
                .unwrap_or(source::REPOSITORY.into()),
            version: env::var("AUBIO_VERSION")
                .unwrap_or(source::VERSION.into()),
        };

        let out_dir = env::var("OUT_DIR")
            .expect("The OUT_DIR is set by cargo.");

        let out_dir = Path::new(&out_dir);

        let src_dir = out_dir.join("source")
            .join(&src.version);

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
        pub repository: String,
        pub version: String,
    }

    pub fn fetch_source(src: &Source, out_dir: &Path) {
        use fetch_unroll::Fetch;

        if !out_dir.is_dir() {
            let src_url = format!("{repo}{ver}.tar.gz",
                                  repo = src.repository,
                                  ver = src.version);

            eprintln!("Fetch aubio sources from {} to {}",
                      src_url, out_dir.display());

            Fetch::from(src_url).unroll().strip_components(1).to(out_dir)
                .expect("Aubio sources should be fetched");
        }
    }

    pub fn generate_bindings(inc_dir: &Path, out_file: &Path) {
        let bindings = bindgen::Builder::default()
            .detect_include_paths(true)
            .clang_args(&[
                format!("-I{}", inc_dir.display()),
            ])
            .header(inc_dir.join("aubio.h").display().to_string())
            .generate()
            .expect("Generated bindings");

        bindings
            .write_to_file(out_file)
            .expect("Writen bindings");
    }
}
