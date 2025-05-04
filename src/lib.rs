use anyhow::Result;
use pelite::{
    pe64::{Pe, PeFile},
    FileMap,
};
use std::{fs, path::Path, process::Command};
use tera::{Context, Tera};

#[derive(serde::Serialize)]
struct Export {
    name: String,
    signature: String,
    ret: String,
}

fn write_and_format<P: AsRef<Path>>(path: &P, contents: &str) -> std::io::Result<()> {
    fs::write(path, contents)?;

    let status = Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .arg(path.as_ref())
        .status()
        .expect("failed to spawn rustfmt");

    if !status.success() {
        eprintln!("warning: rustfmt returned non-zero status");
    }

    Ok(())
}

pub fn generate_proxy<P: AsRef<Path>>(dll_path: P, tpl_path: P, out_path: P) -> Result<()> {
    let dll_path = dll_path.as_ref();
    let tpl_path = tpl_path.as_ref();
    let out_path = out_path.as_ref();

    let file_map = FileMap::open(&dll_path)?;
    let pe = PeFile::from_bytes(file_map.as_ref())?;
    let pe_exports = pe.exports()?;
    let by = pe_exports.by()?;

    let mut exports: Vec<Export> = Vec::new();

    for result in by.iter_names() {
        if let (Ok(name), Ok(_export)) = result {
            println!("Found Function: {}", name);
            //TODO: determine if function even has a signature??
            exports.push(Export {
                name: name.to_string(),
                signature: "tbd: u32".to_owned(),
                ret: "u32".to_string(),
            })
        }
    }

    let mut tera = Tera::default();
    tera.add_template_file(tpl_path, Some("hook"))?;

    let dll_filename = dll_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("orig.dll");

    let mut ctx = Context::new();
    ctx.insert("dll_filename", dll_filename);
    ctx.insert("exports", &exports);

    let rendered = tera.render("hook", &ctx)?;
    write_and_format(&out_path, &rendered)?;
    Ok(())
}
