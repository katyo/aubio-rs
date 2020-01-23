#[cfg(any(/*feature = "generate-bindings", */feature = "compile-library"))]
use git2::build::RepoBuilder;

#[cfg(feature = "fetch-prebuilt")]
use fetch_unroll;

#[cfg(feature = "generate-bindings")]
use bindgen;

use std::process::{
    Command,
    Output,
};

use std::{
    env,
    path::{Path, PathBuf},
    fs::metadata,
    str::from_utf8,
};

enum LinkArg {
    SearchPath(String),
    StaticLib(String),
    SharedLib(String),
}

use self::LinkArg::*;

fn main() {
    if !env::var("CARGO_FEATURE_RUSTDOC").is_ok() {
        let out_dir = PathBuf::from(
            env::var("OUT_DIR").expect("OUT_DIR is set by cargo.")
        );

        let pkg_name = env::var("AUBIO_PKG").ok()
            .unwrap_or_else(|| "aubio".into());

        #[cfg(any(/*feature = "generate-bindings", */feature = "compile-library"))]
        let aubio_src = {
            let aubio_src = out_dir.join("aubio-src");

            // TODO: check contents
            if !metadata(aubio_src.join(".git"))
                .map(|meta| meta.is_dir())
                .unwrap_or(false) {
                    fetch_aubio(&aubio_src);
                }

            aubio_src
        };

        // compiling aubio library and binding extensions
        #[cfg(feature = "compile-library")]
        let link_args = compile_library(&aubio_src);

        // select precompiled aubio library for specified target
        #[cfg(not(feature = "compile-library"))]
        let link_args = select_library(&pkg_name, &out_dir);

        for link_arg in link_args {
            match link_arg {
                SearchPath(path) => println!("cargo:rustc-link-search=native={}", path),
                StaticLib(name) => println!("cargo:rustc-link-lib=static={}", name),
                SharedLib(name) => println!("cargo:rustc-link-lib={}", name),
            }
        }

        #[cfg(feature = "generate-bindings")]
        {
            #[cfg(feature = "compile-library")]
            let aubio_includedir = {
                aubio_src.join("src")
            };

            #[cfg(not(feature = "compile-library"))]
            let aubio_includedir = {
                guess_includedir_from_env()
                    .or_else(|| guess_includedir_using_pkgconfig(&pkg_name))
                    .expect("Unable to determine aubio include directory. You can set it manually using AUBIO_INCLUDEDIR environment variable.")
            };

            let out_file = out_dir.join("bindings.rs");
            generate_bindings(&aubio_includedir, &out_file);
        }
    }
}

#[cfg(not(feature = "compile-library"))]
fn guess_includedir_from_env() -> Option<PathBuf> {
    env::var("AUBIO_INCLUDEDIR").ok().and_then(|dir| {
        let include_dir = Path::new(&dir);

        for &include_dir in &[include_dir, &include_dir.join("aubio")] {
            if metadata(include_dir.join("aubio.h"))
                .map(|meta| meta.is_file()).unwrap_or(false) {
                    return Some(include_dir.to_owned());
                }
        }

        None
    })
}

#[cfg(not(feature = "compile-library"))]
fn lib_ext() -> &'static str {
    #[cfg(not(feature = "dynamic-link"))]
    {
        #[cfg(target_os = "windows")]
        { ".lib" }

        #[cfg(not(target_os = "windows"))]
        { ".a" }
    }

    #[cfg(feature = "dynamic-link")]
    {
        #[cfg(target_os = "windows")]
        { ".dll" }

        #[cfg(not(target_os = "windows"))]
        { ".so" }
    }
}

#[cfg(not(feature = "compile-library"))]
fn guess_libdir_and_lib_from_env() -> (Option<PathBuf>, Option<String>) {
    let lib_name_from_env = env::var("AUBIO_LIB").ok();

    let lib_name = lib_name_from_env.as_ref()
        .map(|lib_name| lib_name.to_owned())
        .unwrap_or_else(|| "aubio".into());

    // Determining library directory
    let lib_dir_from_env = env::var("AUBIO_LIBDIR")
        .ok().and_then(|lib_dir| {
            let lib_dir = Path::new(&lib_dir);
            let lib_path = lib_dir.join(format!("lib{}{}", lib_name, lib_ext()));

            if metadata(&lib_path).map(|meta| meta.is_file()).unwrap_or(false) {
                Some(lib_dir.to_owned())
            } else {
                eprintln!("Warning: library '{}' not found", lib_path.display());
                None
            }
        });

    (lib_dir_from_env, lib_name_from_env)
}

#[cfg(not(feature = "compile-library"))]
fn guess_includedir_using_pkgconfig<S: AsRef<str>>(pkg_name: S) -> Option<PathBuf> {
    let pkg_name = pkg_name.as_ref();
    match Command::new("pkg-config").arg("--cflags").arg(&pkg_name).output() {
        Err(error) => {
            eprintln!("Warning: Unable to execute `pkg-config --cflags {}` due to: {}", pkg_name, error);
        },
        Ok(Output { status, stdout, stderr }) => {
            if !status.success() {
                eprintln!("Warning: Unable to guess cflags for '{}'.", pkg_name);
                eprintln!("pkg-config stderr:");
                eprintln!("{}", from_utf8(stderr.as_slice())
                          .unwrap_or("<invalid UTF8 string>"));
            } else {
                if let Ok(stdout) = from_utf8(stdout.as_slice()) {
                    let mut include_dir = None;
                    'top: for arg in stdout.split_whitespace() {
                        if arg.starts_with("-I") {
                            let dir = Path::new(&arg[2..]);
                            let candidates = &[dir, &dir.join(pkg_name)];

                            for &dir in candidates {
                                if metadata(dir.join("aubio.h"))
                                    .map(|meta| meta.is_file()).unwrap_or(false) {
                                        include_dir = dir.to_owned().into();
                                        break 'top;
                                    }
                            }
                            eprintln!("Warning: Unable to guess include dir for '{}' from candidates:", pkg_name);
                            for dir in candidates {
                                eprintln!("{}", dir.display());
                            }
                        }
                    }
                    return include_dir;
                } else {
                    eprintln!("Warning: Unable to guess include dir for '{}'.", pkg_name);
                    eprintln!("pkg-config stdout: <invalid UTF8 string>");
                }
            }
        },
    }
    None
}

#[cfg(not(feature = "compile-library"))]
fn guess_libdir_and_lib_using_pkgconfig<S: AsRef<str>>(pkg_name: S) -> (Option<PathBuf>, Option<String>) {
    let pkg_name = pkg_name.as_ref();
    match Command::new("pkg-config").arg("--libs").arg(&pkg_name).output() {
        Err(error) => {
            eprintln!("Warning: Unable to execute `pkg-config --libs {}` due to: {}", pkg_name, error);
        },
        Ok(Output { status, stdout, stderr }) => {
            if !status.success() {
                eprintln!("Warning: Unable to guess libs for '{}'.", pkg_name);
                eprintln!("pkg-config stderr:");
                eprintln!("{}", from_utf8(stderr.as_slice())
                          .unwrap_or("<invalid UTF8 string>"));
            } else {
                if let Ok(stdout) = from_utf8(stdout.as_slice()) {
                    let mut lib_dir = None;
                    let mut lib_name = None;
                    for arg in stdout.split_whitespace() {
                        if arg.starts_with("-L") {
                            lib_dir = Path::new(&arg[2..]).to_owned().into();
                        }
                        if arg.starts_with("-l") {
                            lib_name = arg[2..].to_owned().into();
                        }
                    }
                    return (lib_dir, lib_name);
                } else {
                    eprintln!("Warning: Unable to guess libs for '{}'.", pkg_name);
                    eprintln!("pkg-config stdout: <invalid UTF8 string>");
                }
            }
        },
    }
    (None, None)
}

#[cfg(not(feature = "compile-library"))]
fn select_library<S: AsRef<str>>(pkg_name: S, out_dir: &Path) -> Vec<LinkArg> {
    let lib_name = "aubio";

    let (libdir_from_env, lib_from_env) = guess_libdir_and_lib_from_env();
    let (libdir_from_pkg, lib_from_pkg) = guess_libdir_and_lib_using_pkgconfig(&pkg_name);

    let (lib_dir, lib_name) = if let Some(lib_dir) = libdir_from_env {
        (lib_dir, lib_from_env.unwrap_or_else(|| lib_name.into()))
    } else if let Some(lib_dir) = libdir_from_pkg {
        (lib_dir, lib_from_pkg.or_else(|| lib_from_env).unwrap_or_else(|| lib_name.into()))
    } else {
        #[cfg(feature = "fetch-prebuilt")]
        {
            let target_arch = env::var("CARGO_CFG_TARGET_ARCH")
                .expect("CARGO_CFG_TARGET_ARCH is set by cargo.");

            let profile = env::var("PROFILE")
                .expect("PROFILE is set by cargo.");

            let lib_arch = rustc_target(&target_arch);

            let lib_dir = out_dir.join(lib_arch);

            // TODO: fetch prebuilt

            (lib_dir, lib_name.into())
        }

        #[cfg(not(feature = "download-prebuilt"))]
        {
            panic!("Warning: Unable to search prebuilt library for '{}'. You can set valid AUBIO_LIBDIR environment variable or enable 'download-prebuilt' feature.", pkg_name.as_ref());
        }
    };

    vec![
        SearchPath(lib_dir.display().to_string()),
        if cfg!(feature = "dynamic-link") { SharedLib(lib_name) } else { StaticLib(lib_name) },
    ]
}

#[cfg(feature = "compile-library")]
fn compile_library(lib_src: &Path) -> Vec<LinkArg> {
    let lib_name = String::from("aubio");

    /*
WAF Options:
  --build-type=BUILD_TYPE
                        whether to compile with (--build-type=release) or without (--build-type=debug) compiler opimizations
                        [default: release]
  --debug               build in debug mode (see --build-type)
  --enable-fftw3f       compile with fftw3f instead of ooura (recommended)
  --disable-fftw3f      do not compile with fftw3f
  --enable-fftw3        compile with fftw3 instead of ooura
  --disable-fftw3       do not compile with fftw3
  --enable-intelipp     use Intel IPP libraries (auto)
  --disable-intelipp    do not use Intel IPP libraries
  --enable-complex      compile with C99 complex
  --disable-complex     do not use C99 complex (default)
  --enable-jack         compile with jack (auto)
  --disable-jack        disable jack support
  --enable-sndfile      compile with sndfile (auto)
  --disable-sndfile     disable sndfile
  --enable-avcodec      compile with libavcodec (auto)
  --disable-avcodec     disable libavcodec
  --enable-samplerate   compile with samplerate (auto)
  --disable-samplerate  disable samplerate
  --enable-memcpy       use memcpy hacks (default)
  --disable-memcpy      do not use memcpy hacks
  --enable-double       compile in double precision mode
  --disable-double      compile in single precision mode (default)
  --enable-fat          build fat binaries (darwin only)
  --disable-fat         do not build fat binaries (default)
  --enable-accelerate   use Accelerate framework (darwin only) (auto)
  --disable-accelerate  do not use Accelerate framework
  --enable-apple-audio  use CoreFoundation (darwin only) (auto)
  --disable-apple-audio
                        do not use CoreFoundation framework
  --enable-blas         use BLAS acceleration library (no)
  --disable-blas        do not use BLAS library
  --enable-atlas        use ATLAS acceleration library (no)
  --disable-atlas       do not use ATLAS library
  --enable-wavread      compile with source_wavread (default)
  --disable-wavread     do not compile source_wavread
  --enable-wavwrite     compile with source_wavwrite (default)
  --disable-wavwrite    do not compile source_wavwrite
  --enable-docs         build documentation (auto)
  --disable-docs        do not build documentation
  --enable-tests        build tests (true)
  --disable-tests       do not build or run tests
  --enable-examples     build examples (true)
  --disable-examples    do not build examples
  --with-target-platform=TARGET_PLATFORM
                        set target platform for cross-compilation
  --notests             Exec no unit tests
  --alltests            Exec all unit tests
  --clear-failed        Force failed unit tests to run again next time
  --testcmd=TESTCMD     Run the unit tests using the test-cmd string example "--testcmd="valgrind --error-exitcode=1 %s" to run
                        under valgrind
  --dump-test-scripts   Create python scripts to help debug tests

  Configuration options:
    -o OUT, --out=OUT   build dir for the project
    -t TOP, --top=TOP   src dir for the project
    --check-c-compiler=CHECK_C_COMPILER
                        list of C compilers to try [gcc clang icc]
     */

    let mut wafopts = String::new();

    wafopts.push_str(" --build-type=");
    wafopts.push_str(&env::var("PROFILE").expect("PROFILE env var is set by cargo."));

    let flags = [
        ("docs", false),
        ("tests", false),
        ("examples", false),

        ("fftw3f", cfg!(feature = "with-fftw3f")),
        ("fftw3", cfg!(feature = "with-fftw3")),

        ("wavread", cfg!(feature = "with-wav")),
        ("wavwrite", cfg!(feature = "with-wav")),

        ("jack", cfg!(feature = "with-jack")),
        ("sndfile", cfg!(feature = "with-sndfile")),
        ("avcodec", cfg!(feature = "with-avcodec")),
        ("samplerate", cfg!(feature = "with-samplerate")),
    ];

    for &(flag, state) in &flags {
        wafopts.push_str(if state { " --enable-" } else { " --disable-" });
        wafopts.push_str(flag);
    }

    match Command::new("make").current_dir(lib_src).env("WAFOPTS", wafopts).output() {
        Err(error) => {
            panic!("Error: Unable to execute `make` to build '{}' library due to: {}", lib_name, error);
        },
        Ok(Output { status, stderr, .. }) => {
            if !status.success() {
                panic!("Error: Compilation errors when building '{}' library: {}", lib_name,
                       from_utf8(stderr.as_slice()).unwrap_or("<invalud UTF8 string>"));
            }
        }
    }

    vec![
        SearchPath(format!("{}/build/src", lib_src.display())),
        if cfg!(feature = "static-link") { StaticLib(lib_name) } else { SharedLib(lib_name) },
    ]
}

#[cfg(any(/*feature = "generate-bindings", */feature = "compile-library"))]
fn fetch_aubio(out_dir: &Path) { // clonning aubio git repo
    //let repo = "https://github.com/aubio/aubio";
    //let version = "master";
    let repo = "https://github.com/katyo/aubio";
    let version = "fix-shebang";

    let url = env::var("AUBIO_GIT_URL")
        .unwrap_or_else(|_| repo.into());
    let tag = env::var("AUBIO_GIT_TAG")
        .unwrap_or_else(|_| version.into());

    let _repo = match RepoBuilder::new()
        .branch(&tag)
        .clone(&url, out_dir) {
            Ok(repo) => repo,
            Err(error) => panic!("Unable to fetch 'aubio' library from git due to {}. url={} tag={}", error, url, tag),
        };
}

#[cfg(not(feature = "compile-library"))]
fn rustc_target<S: AsRef<str>>(target_arch: &S) -> &'static str {
    match target_arch.as_ref() {
        "arm" => "armv7",
        "aarch64" => "aarch64",
        "x86" => "i686",
        "x86_64" => "x86_64",
        arch => panic!("Unsupported architecture {}", arch),
    }
}

#[cfg(feature = "generate-bindings")]
fn android_target<S: AsRef<str>>(target_arch: &S) -> &'static str {
    match target_arch.as_ref() {
        "arm" => "arm-linux-androideabi",
        "aarch64" => "aarch64-linux-android",
        "x86" => "i686-linux-android",
        "x86_64" => "x86_64-linux-android",
        arch => panic!("Unsupported architecture {}", arch),
    }
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(aubio_includedir: &Path, out_file: &Path) {
    let target_os = env::var("CARGO_CFG_TARGET_OS")
        .expect("CARGO_CFG_TARGET_OS is set by cargo.");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH")
        .expect("CARGO_CFG_TARGET_ARCH is set by cargo.");

    let mut clang_args = Vec::new();

    if target_os == "android" {
        let ndk_target = android_target(&target_arch);

        clang_args.push(format!("--target={}", ndk_target));
    }

    let bindings = bindgen::Builder::default()
        .detect_include_paths(true)
        .clang_args(&clang_args)
        .clang_args(&[
            format!("-I{}", aubio_includedir.display()),
        ])
        .header(aubio_includedir.join("aubio.h").display().to_string())
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}
