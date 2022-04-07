use aya_gen::btf_types;
use std::{ fs::File, io::Write, path::{Path,PathBuf}};

pub fn generate() -> Result<(), anyhow::Error> {
    let dir = PathBuf::from("albert-aya1-ebpf/src");
    let names: Vec<&str> = vec!["ethhdr", "iphdr"];
    let bindings = btf_types::generate(Path::new("/sys/kernel/btf/vmlinux"), &names, true)?;
    let mut out = File::create(dir.join("bindings.rs"))?;
    write!(out, "{}", bindings)?;

    Ok(())
}
