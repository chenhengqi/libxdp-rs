use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let out_dir_str = out_dir.to_str().unwrap();

    if cfg!(target_os = "linux") {
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

        let status = Command::new("mv")
            .arg("libbpf.a").arg(out_dir_str)
            .current_dir(src_dir.join("xdp-tools/lib/libbpf/src"))
            .status()
            .unwrap();

        assert!(status.success());

        let status = Command::new("mv")
            .arg("libxdp.a").arg(out_dir_str)
            .current_dir(src_dir.join("xdp-tools/lib/libxdp"))
            .status()
            .unwrap();

        assert!(status.success());

        println!("cargo:rustc-link-search=native={}", out_dir_str);
        println!("cargo:rustc-link-lib=elf");
        println!("cargo:rustc-link-lib=z");
        println!("cargo:rustc-link-lib=static=bpf");
        println!("cargo:rustc-link-lib=static=xdp");
    }
}
