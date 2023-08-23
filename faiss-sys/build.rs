use anyhow::{Context, Result};
use std::env;

fn main() -> Result<()> {
    #[cfg(feature = "static")]
    static_link_faiss()?;
    #[cfg(not(feature = "static"))]
    println!("cargo:rustc-link-lib=faiss_c");
    Ok(())
}

#[cfg(feature = "static")]
fn static_link_faiss() -> Result<()> {
    let mut cfg = cmake::Config::new("faiss");
    cfg.define("FAISS_ENABLE_C_API", "ON")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("FAISS_ENABLE_GPU", "OFF")
        .define("FAISS_ENABLE_PYTHON", "OFF")
        .define("BUILD_TESTING", "OFF")
        .very_verbose(true);
    let dst = cfg.build();
    let faiss_location = dst.join("lib");
    let faiss_c_location = dst.join("build/c_api");
    println!(
        "cargo:rustc-link-search=native={}",
        faiss_location.display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        faiss_c_location.display()
    );
    println!("cargo:rustc-link-lib=static=faiss_c");
    println!("cargo:rustc-link-lib=static=faiss");
    link_cxx();

    let target = env::var("TARGET").context("TARGET variable not set")?;

    if !target.contains("msvc") && !target.contains("apple") {
        println!("cargo:rustc-link-lib=gomp");
    } else {
        println!("cargo:rustc-link-lib=omp");
    }
    println!("cargo:rustc-link-lib=blas");
    println!("cargo:rustc-link-lib=lapack");
    Ok(())
}

#[cfg(feature = "static")]
fn link_cxx() {
    let cxx = match std::env::var("CXXSTDLIB") {
        Ok(s) if s.is_empty() => None,
        Ok(s) => Some(s),
        Err(_) => {
            let target = std::env::var("TARGET").unwrap();
            if target.contains("msvc") {
                None
            } else if target.contains("apple")
                | target.contains("freebsd")
                | target.contains("openbsd")
            {
                Some("c++".to_string())
            } else {
                Some("stdc++".to_string())
            }
        }
    };
    if let Some(cxx) = cxx {
        println!("cargo:rustc-link-lib={}", cxx);
    }
}
