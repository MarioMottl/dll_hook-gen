use std::path::{Path, PathBuf};

use clap::Parser;
use dll_hook_gen::generate_proxy;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    dll_path: String,

    #[arg(
        short,
        long,
        default_value = "./templates/hook.rs.tera",
        required = false
    )]
    template_path: String,

    #[arg(short, long)]
    out_path: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let dll_path = Path::new(&args.dll_path);
    let mut out_path = PathBuf::from(&args.out_path);
    let template_path = Path::new(&args.template_path);
    if !dll_path.exists() {
        println!("Provided <dll_path> does not exist");
    }
    if out_path.is_dir() {
        out_path.push("out.rs");
    }
    if !template_path.exists() {
        println!("Provided <template_path> does not exist");
    }

    generate_proxy(dll_path, template_path, &out_path)?;
    Ok(())
}
