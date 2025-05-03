use dll_hook_gen::generate_proxy;

fn main() -> anyhow::Result<()> {
    //TODO: use clap for command line argument parsing
    generate_proxy(
        "/home/mario/dev/dll-hook-gen/test/testlib.dll",
        "./templates/hook.rs.tera",
        "out.rs",
    )?;
    Ok(())
}
