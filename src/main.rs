use clap::Parser;
use rmcloud::ServerBuilder;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sets a custom config file
    #[clap(
        short,
        long,
        value_parser,
        value_name = "FILE",
        default_value = "./config.yaml"
    )]
    config_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!(
        r#"//////////////////////////////////////////////////////////////////////
//   _______   __       __   ______   __                            __ 
//  |       \ |  \     /  \ /      \ |  \                          |  \
//  | $$$$$$$\| $$\   /  $$|  $$$$$$\| $$  ______   __    __   ____| $$
//  | $$__| $$| $$$\ /  $$$| $$   \$$| $$ /      \ |  \  |  \ /      $$
//  | $$    $$| $$$$\  $$$$| $$      | $$|  $$$$$$\| $$  | $$|  $$$$$$$
//  | $$$$$$$\| $$\$$ $$ $$| $$   __ | $$| $$  | $$| $$  | $$| $$  | $$
//  | $$  | $$| $$ \$$$| $$| $$__/  \| $$| $$__/ $$| $$__/ $$| $$__| $$
//  | $$  | $$| $$  \$ | $$ \$$    $$| $$ \$$    $$ \$$    $$ \$$    $$
//   \$$   \$$ \$$      \$$  \$$$$$$  \$$  \$$$$$$   \$$$$$$   \$$$$$$$     
//                                                                     
//                                                                     
//  written by Heiss in Rust
//  Issues: https://github.com/Heiss/rust-remarkablecloud
//
//////////////////////////////////////////////////////////////////////
"#
    );

    ServerBuilder::new(args.config_path).build()?.execute()?;

    Ok(())
}
