mod source {
    //pub const REPOSITORY: &str = "https://github.com/aubio/aubio";
    //pub const VERSION: &str = "0.4.9";
    pub const REPOSITORY: &str = "https://github.com/katyo/aubio";
    pub const VERSION: &str = "master";
}

fn main() {
    #[cfg(not(feature = "rustdoc"))]
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

        utils::fix_source(&src_dir);

        utils::compile_library(&src_dir);
    }
}

mod utils {
    use std::{
        env,
        path::Path,
    };

    pub struct Source {
        pub repository: String,
        pub version: String,
    }

    pub fn fetch_source(src: &Source, out_dir: &Path) {
        use fetch_unroll::Fetch;

        if !out_dir.is_dir() {
            let src_url = format!("{repo}/archive/{ver}.tar.gz",
                                  repo = src.repository,
                                  ver = src.version);

            eprintln!("Fetch fluidlite from {} to {}",
                      src_url, out_dir.display());

            Fetch::from(src_url).unroll().strip_components(1).to(out_dir)
                .expect("FluidLite sources should be fetched.");
        }
    }

    pub fn fix_source(src_dir: &Path) {
        use std::{
            io::{Read, Write},
            fs::File,
        };

        let scripts = src_dir.join("scripts");
        for script in &["get_waf.sh", "build_mingw", "build_android", "build_emscripten"] {
            let script = scripts.join(script);
            let mut source = String::new();
            File::open(&script).unwrap().read_to_string(&mut source).unwrap();
            if source.starts_with("#! /bin/bash") {
                File::create(&script).unwrap().write(source.replace("#! /bin/bash", "#!/usr/bin/env bash").as_bytes()).unwrap();
            }
        }
    }

    pub fn compile_library(src_dir: &Path) {
        use std::{
            str::from_utf8,
            process::{Command, Output},
        };

        let lib_name = String::from("aubio");

        let target = env::var("TARGET")
            .expect("The TARGET is set by cargo.");

        let profile = env::var("PROFILE")
            .expect("The PROFILE is set by cargo.");

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

        if profile == "debug" {
            wafopts.push_str(" --debug");
        }

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

        let mut toolchain_env = Vec::new();

        // For cargo: like "CARGO_TARGET_I686_LINUX_ANDROID_CC".  This is really weakly
        // documented; see https://github.com/rust-lang/cargo/issues/5690 and follow
        // links from there.

        // For build.rs in `cc` consumers: like "CC_i686-linux-android". See
        // https://github.com/alexcrichton/cc-rs#external-configuration-via-environment-variables.

        if let Ok(cc) = env::var(format!("CARGO_TARGET_{}_CC", target))
            .or_else(|_| env::var(format!("CC_{}", target))) {
                toolchain_env.push(("CC", cc));
            }
        if let Ok(ar) = env::var(format!("CARGO_TARGET_{}_AR", target))
            .or_else(|_| env::var(format!("AR_{}", target))) {
                toolchain_env.push(("AR", ar));
            }
        if let Ok(ld) = env::var(format!("CARGO_TARGET_{}_LINKER", target)) {
            toolchain_env.push(("LINKER", ld));
        }

        match Command::new("make")
            .current_dir(src_dir)
            .env("WAFOPTS", wafopts)
            .envs(toolchain_env)
            .output()
        {
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

        println!("cargo:rustc-link-search=native={}", src_dir.join("build").join("src").display());

        #[cfg(feature = "shared")]
        println!("cargo:rustc-link-lib={}", lib_name);

        #[cfg(not(feature = "shared"))]
        println!("cargo:rustc-link-lib=static={}", lib_name);
    }
}
