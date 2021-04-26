use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let libbpf_src_dir = src_dir.join("xdp-tools/lib/libbpf/src");
    let libbpf_lib_dir = libbpf_src_dir.to_str().unwrap();
    let libxdp_src_dir = src_dir.join("xdp-tools/lib/libxdp");
    let libxdp_lib_dir = libxdp_src_dir.to_str().unwrap();

    if cfg!(target_os = "linux") {
        // run `git submodule update`
        let status = Command::new("git")
            .arg("submodule")
            .arg("update")
            .current_dir(src_dir.join("xdp-tools"))
            .status()
            .unwrap();

        assert!(status.success());

        // run `configure`
        let status = Command::new("./configure")
            .current_dir(src_dir.join("xdp-tools"))
            .status()
            .unwrap();

        assert!(status.success());

        // build libbpf/libxdp
        let status = Command::new("make")
            .arg("lib")
            .current_dir(src_dir.join("xdp-tools"))
            .status()
            .unwrap();

        assert!(status.success());

        println!("cargo:rustc-link-lib=elf");
        println!("cargo:rustc-link-lib=z");
        println!("cargo:rustc-link-search=native={}", libbpf_lib_dir);
        println!("cargo:rustc-link-lib=static=bpf");
        println!("cargo:rustc-link-search=native={}", libxdp_lib_dir);
        println!("cargo:rustc-link-lib=static=xdp");
    }
}
