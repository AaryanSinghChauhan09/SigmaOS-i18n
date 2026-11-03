use clap::Parser;
use i18n::i18n;

#[derive(Parser)]
struct Cli;

fn main() -> anyhow::Result<()> {
    let config = i18n::Config::default();
    let mut component = i18n::new(config);
    component.initialize()?;
    println!("SigmaOS-i18n ready");
    Ok(())
}
