use cli::{CLIError, CliArgs, CLI};
use rmcloud::ServerBuilder;
use storage::{CodeLocalStorage, UserLocalStorage};

fn main() -> anyhow::Result<()> {
    // Here you can specify, which storage should be used.
    // TODO: the Storage should be handled by the config.toml instead of this thingy here
    let (args, user_storage, code_storage): (
        CliArgs,
        Box<UserLocalStorage>,
        Box<CodeLocalStorage>,
    ) = match CLI::parse_args() {
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

    ServerBuilder::new(args.config_path, user_storage, code_storage)
        .build()?
        .execute()?;

    Ok(())
}
