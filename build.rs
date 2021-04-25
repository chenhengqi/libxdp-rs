use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let out_dir_str = out_dir.to_str().unwrap();

    if cfg!(target_os = "linux") {
        let status = Command::new("./configure")
            .current_dir(src_dir.join("xdp-tools"))
            .status()
            .unwrap();

        assert!(status.success());

        let status = Command::new("make")
            .arg("lib")
            .env("BUILD_STATIC_ONLY", "y")
            .env("PREFIX", "/")
            .env("LIBDIR", "")
            .env("OBJDIR", out_dir.join("obj").to_str().unwrap())
            .env("DESTDIR", out_dir_str)
            .env("CFLAGS", "-g -O2 -Werror -Wall -fPIC")
            .current_dir(src_dir.join("xdp-tools"))
            .status()
            .unwrap();

        assert!(status.success());

        println!("cargo:rustc-link-search=native={}", out_dir_str);
        println!("cargo:rustc-link-lib=elf");
        println!("cargo:rustc-link-lib=z");
        println!("cargo:rustc-link-lib=static=bpf");
        println!("cargo:rustc-link-lib=static=xdp");
        println!("cargo:include={}/include", out_dir_str);
    }
}
