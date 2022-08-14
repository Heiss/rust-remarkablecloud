use cli::{CLIError, CliArgs, Storages, CLI};
use rmcloud::ServerBuilder;
use storage::{CodeLocalStorage, UserLocalStorage};

fn main() -> anyhow::Result<()> {
    // Here you can specify, which storage should be used.
    // TODO: the Storage should be handled by the config.yaml instead of this thingy here
    let (args, _storages): (CliArgs, Storages<UserLocalStorage, CodeLocalStorage>) =
        match CLI::parse_args() {
            Ok(v) => v,
            Err(CLIError::CommandFound) => return Ok(()), // hide the error, if CLI process something successfully
            Err(v) => return Err(v.into()),
        };

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
