#![allow(dead_code)]

use std::path::Path;

const LIB_NAME: &str = "aubio";
const LIB_VER: &str = "0.4.9";

fn main() {
    use std::env;

    #[cfg(any(not(feature = "bindgen"), feature = "update-bindings"))]
    fn bindings_filename() -> String {
        format!(
            "{}-{}-{}.rs",
            env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
            env::var("CARGO_CFG_TARGET_OS").unwrap(),
            env::var("CARGO_CFG_TARGET_ENV").unwrap()
        )
    }

    #[cfg(any(not(feature = "bindgen"), feature = "update-bindings"))]
    fn bindings_filepath(filename: &str) -> impl AsRef<Path> {
        Path::new("src").join("bindings").join(filename)
    }

    #[cfg(not(feature = "bindgen"))]
    {
        let bindings_file = bindings_filename();

        if bindings_filepath(&bindings_file).as_ref().is_file() {
            println!("cargo:rustc-env=AUBIO_BINDINGS={}", bindings_file);
        } else {
            panic!("No prebuilt bindings. Try use `bindgen` feature.",);
        }
    }

    let out_dir = env::var("OUT_DIR").expect("The OUT_DIR is set by cargo.");
    let out_dir = Path::new(&out_dir);

    let src_dir = Path::new("aubio");

    #[cfg(feature = "bindgen")]
    {
        let inc_dirs = try_find_library_inc_dirs().unwrap_or_else(|| vec![src_dir.join("src")]);

        let bindings = out_dir.join("bindings.rs");

        generate_bindings(inc_dirs, &bindings);

        #[cfg(feature = "update-bindings")]
        {
            let out_path = bindings_filepath(&bindings_filename());
            update_bindings(&bindings, &out_path);
        }
    }

    if !try_find_and_use_library() {
        let lib_dir = out_dir;

        build_library(src_dir, lib_dir);
        add_lib_path(lib_dir);
        add_lib(LIB_NAME, !cfg!(not(feature = "shared")));
    }
}

#[cfg(feature = "bindgen")]
fn generate_bindings<P: AsRef<Path>>(
    inc_dirs: impl IntoIterator<Item = P>,
    out_file: impl AsRef<Path>,
) {
    let bindings = bindgen::Builder::default()
        .detect_include_paths(true)
        .clang_args(
            inc_dirs
                .into_iter()
                .map(|dir| format!("-I{}", dir.as_ref().display())),
        )
        .header_contents("library.h", "#include <aubio.h>")
        .generate()
        .expect("Generated bindings.");

    bindings.write_to_file(out_file).expect("Written bindings.");
}

#[cfg(feature = "update-bindings")]
fn update_bindings(bind_file: impl AsRef<Path>, dest_file: impl AsRef<Path>) {
    use std::{env, fs, io::Write};

    let dest_file = dest_file.as_ref();

    fs::create_dir_all(&dest_file.parent().unwrap()).unwrap();
    fs::copy(&bind_file, &dest_file).unwrap();

    if let Ok(github_env) = env::var("GITHUB_ENV") {
        let mut env_file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(github_env)
            .unwrap();
        writeln!(env_file, "AUBIO_SYS_BINDINGS_FILE={}", dest_file.display()).unwrap();
    }
}

fn add_lib(name: &str, static_: bool) {
    println!(
        "cargo:rustc-link-lib={}{}",
        if static_ { "static=" } else { "" },
        name
    );
}

fn add_lib_path(path: impl AsRef<Path>) {
    println!("cargo:rustc-link-search={}", path.as_ref().display());
}

#[cfg(feature = "pkg-config")]
fn rust_use_pkg(pkg: &pkg_config::Library) {
    for path in &pkg.link_paths {
        add_lib_path(path);
    }
    for lib in &pkg.libs {
        add_lib(lib, cfg!(feature = "static"));
    }
}

#[cfg(feature = "pkg-config")]
fn cc_use_pkg(build: &mut cc::Build, pkg: &pkg_config::Library) {
    for (k, v) in &pkg.defines {
        if let Some(v) = v {
            build.define(k, v.as_ref());
        } else {
            build.define(k, None);
        }
    }
    build.includes(&pkg.include_paths);
    rust_use_pkg(&pkg);
}

#[cfg(feature = "pkg-config")]
fn find_pkgs<S: AsRef<str>, V: AsRef<str>>(
    libs: impl IntoIterator<Item = (S, V)>,
) -> Option<Vec<pkg_config::Library>> {
    libs.into_iter()
        .map(|(name, version)| {
            pkg_config::Config::new()
                .atleast_version(version.as_ref())
                .probe(name.as_ref())
        })
        .collect::<Result<Vec<_>, _>>()
        .ok()
}

#[cfg(not(feature = "pkg-config"))]
fn try_find_and_use_pkgs<S: AsRef<str>, V: AsRef<str>>(
    _build: &mut cc::Build,
    _libs: impl IntoIterator<Item = (S, V)>,
) -> bool {
    false
}

#[cfg(feature = "pkg-config")]
fn try_find_and_use_pkgs<S: AsRef<str>, V: AsRef<str>>(
    build: &mut cc::Build,
    libs: impl IntoIterator<Item = (S, V)>,
) -> bool {
    find_pkgs(libs)
        .map(|pkgs| {
            for pkg in &pkgs {
                cc_use_pkg(build, pkg);
            }
            true
        })
        .unwrap_or(false)
}

#[cfg(feature = "pkg-config")]
fn find_library() -> Option<pkg_config::Library> {
    // try find system-wide library
    pkg_config::Config::new()
        .atleast_version(LIB_VER)
        .probe(LIB_NAME)
        .ok()
}

fn try_find_and_use_library() -> bool {
    #[cfg(any(feature = "builtin", not(feature = "pkg-config")))]
    {
        false
    }

    #[cfg(all(not(feature = "builtin"), feature = "pkg-config"))]
    {
        find_library()
            .map(|pkg| {
                // Use installed system-wide package
                rust_use_pkg(&pkg);
                true
            })
            .unwrap_or(false)
    }
}

#[cfg(feature = "bindgen")]
fn try_find_library_inc_dirs() -> Option<Vec<std::path::PathBuf>> {
    #[cfg(not(feature = "pkg-config"))]
    {
        None
    }

    #[cfg(feature = "pkg-config")]
    {
        find_library().map(|pkg| pkg.include_paths)
    }
}

fn build_library(src_dir: &Path, lib_dir: &Path) {
    use std::env;

    fn cc_check_with(
        lib_dir: &Path,
        code: impl AsRef<str>,
        with: impl FnOnce(&mut cc::Build),
    ) -> bool {
        use std::fs::{remove_file, write};

        let tmp_src = lib_dir.join("_tmp_.c");
        write(&tmp_src, code.as_ref()).unwrap();

        let mut build = cc::Build::new();
        build.file(&tmp_src);
        with(&mut build);
        let has = build.try_compile("_tmp_").is_ok();

        remove_file(tmp_src).unwrap();
        /*if has {
            remove_file(lib_dir.join("lib_tmp_.a")).unwrap();
        }*/

        has
    }

    fn cc_check(lib_dir: &Path, code: impl AsRef<str>) -> bool {
        cc_check_with(lib_dir, code, |_| {})
    }

    fn has_header(lib_dir: &Path, name: impl AsRef<str>) -> bool {
        cc_check(lib_dir, format!("#include <{}>", name.as_ref()))
    }

    fn with_header(lib_dir: &Path, build: &mut cc::Build, name: impl AsRef<str>) -> bool {
        let def: String = "HAVE_"
            .chars()
            .chain(name.as_ref().chars().map(|c| match c {
                '.' | '/' => '_',
                c => c.to_ascii_uppercase(),
            }))
            .collect();
        let has = has_header(lib_dir, name);
        if has {
            build.define(&def, None);
        }
        has
    }

    let mut build = cc::Build::new();

    let _target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
    let _target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    build.out_dir(lib_dir);
    build.flag_if_supported("-std=c99");

    build.define("HAVE_MEMCPY_HACKS", None);

    with_header(lib_dir, &mut build, "stdlib.h");
    with_header(lib_dir, &mut build, "stdio.h");
    with_header(lib_dir, &mut build, "complex.h");
    with_header(lib_dir, &mut build, "math.h");
    with_header(lib_dir, &mut build, "string.h");
    with_header(lib_dir, &mut build, "errno.h");
    with_header(lib_dir, &mut build, "limits.h");
    with_header(lib_dir, &mut build, "stdarg.h");

    if cc_check(
        lib_dir,
        r#"#include <stdio.h>
           #define AUBIO_ERR(...) fprintf(stderr, __VA_ARGS__)"#,
    ) {
        build.define("HAVE_C99_VARARGS_MACROS", None);
    }

    #[cfg(feature = "fftw3")]
    {
        build.define("HAVE_FFTW3", None);
        build.include(src_dir.join("..").join("fftw"));
        #[cfg(not(feature = "double"))]
        {
            build.define("HAVE_FFTW3F", None);
        }
        add_lib(
            if cfg!(feature = "double") {
                "fftw3"
            } else {
                "fftw3f"
            },
            cfg!(feature = "static"),
        );
    }

    #[cfg(feature = "intelipp")]
    if has_header(&lib_dir, "ippcore.h")
        || has_header(&lib_dir, "ippvm.h")
        || has_header(&lib_dir, "ipps.h")
    {
        build.define("HAVE_INTEL_IPP", None);
        add_lib("ippcore", false);
        add_lib("ippvm", false);
        add_lib("ipps", false);

        if _target_env == "msvc" {
            build.define("_IPP_SEQUENTIAL_STATIC", None);
        }
    }

    #[cfg(feature = "double")]
    build.define("HAVE_AUBIO_DOUBLE", None);

    #[cfg(feature = "accelerate")]
    if _target_vendor == "apple" {
        build.define("HAVE_ACCELERATE", None);
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }

    #[cfg(feature = "blas")]
    {
        let atlas = with_header(lib_dir, &mut build, "atlas/cblas.h");

        if atlas
            || with_header(lib_dir, &mut build, "openblas/cblas.h")
            || with_header(lib_dir, &mut build, "cblas.h")
        {
            build.define("HAVE_BLAS", None);
            add_lib("blas", cfg!(feature = "static"));

            #[cfg(feature = "atlas")]
            if atlas {
                build.define("HAVE_ATLAS", None);
                add_lib("atlas", cfg!(feature = "static"));
            }
        }
    }

    match env::var("PROFILE").unwrap_or_default().as_str() {
        "debug" => {
            build.define("DEBUG", None);
        }
        "release" => {
            build.define("NDEBUG", None);
        }
        _ => (),
    }

    let src_dir = src_dir.join("src");
    build.include(&src_dir);

    build.files(
        [
            "fvec.c",
            "cvec.c",
            "lvec.c",
            "fmat.c",
            "mathutils.c",
            "musicutils.c",
            "vecutils.c",
        ]
        .iter()
        .map(|src| src_dir.join(src)),
    );
    build.files(
        [
            "pitchshift_dummy.c",
            "pitchshift_rubberband.c",
            "rubberband_utils.c",
            "timestretch_dummy.c",
            "timestretch_rubberband.c",
        ]
        .iter()
        .map(|src| src_dir.join("effects").join(src)),
    );
    build.files(
        [
            "pitch.c",
            "pitchfcomb.c",
            "pitchmcomb.c",
            "pitchschmitt.c",
            "pitchspecacf.c",
            "pitchyin.c",
            "pitchyinfast.c",
            "pitchyinfft.c",
        ]
        .iter()
        .map(|src| src_dir.join("pitch").join(src)),
    );
    build.files(
        [
            "awhitening.c",
            "dct.c",
            "dct_accelerate.c",
            "dct_fftw.c",
            "dct_ipp.c",
            "dct_ooura.c",
            "dct_plain.c",
            "fft.c",
            "filterbank.c",
            "filterbank_mel.c",
            "mfcc.c",
            "ooura_fft8g.c",
            "phasevoc.c",
            "specdesc.c",
            "statistics.c",
            "tss.c",
        ]
        .iter()
        .map(|src| src_dir.join("spectral").join(src)),
    );
    build.files(
        ["notes.c"]
            .iter()
            .map(|src| src_dir.join("notes").join(src)),
    );
    build.files(
        ["onset.c", "peakpicker.c"]
            .iter()
            .map(|src| src_dir.join("onset").join(src)),
    );
    build.files(
        ["sampler.c", "wavetable.c"]
            .iter()
            .map(|src| src_dir.join("synth").join(src)),
    );
    build.files(
        ["beattracking.c", "tempo.c"]
            .iter()
            .map(|src| src_dir.join("tempo").join(src)),
    );
    build.files(
        [
            "a_weighting.c",
            "biquad.c",
            "c_weighting.c",
            "filter.c",
            "resampler.c",
        ]
        .iter()
        .map(|src| src_dir.join("temporal").join(src)),
    );
    build.files(
        [
            "hist.c",
            "log.c",
            "parameter.c",
            "scale.c",
            "strutils.c",
            "windll.c",
        ]
        .iter()
        .map(|src| src_dir.join("utils").join(src)),
    );
    build.files(
        [
            "audio_unit.c",
            "ioutils.c",
            "sink.c",
            //"sink_apple_audio.c",
            //"sink_flac.c",
            //"sink_sndfile.c",
            //"sink_vorbis.c",
            //"sink_wavwrite.c",
            "source.c",
            //"source_apple_audio.c",
            //"source_avcodec.c",
            //"source_sndfile.c",
            //"source_wavread.c",
            //"utils_apple_audio.c",
        ]
        .iter()
        .map(|src| src_dir.join("io").join(src)),
    );

    #[cfg(feature = "shared")]
    build.shared_flag(true);

    #[cfg(feature = "static")]
    build.static_flag(true);

    build.compile(LIB_NAME);
}
