use crate::command_prelude::*;

use cargo::ops;

pub fn cli() -> App {
    subcommand("new")
        .about("Create a new cargo package at <path>")
        .arg(opt("quiet", "No output printed to stdout").short("q"))
        .arg(Arg::with_name("path").required(true))
        .arg(opt("registry", "Registry to use").value_name("REGISTRY"))
        .arg_new_opts()
        .after_help("Run `cargo help new` for more detailed information.\n")
}

pub fn exec(config: &mut Config, args: &ArgMatches<'_>) -> CliResult {
    let opts = args.new_options(config)?;

    let package_name = ops::new(&opts, config)?;
    config.shell().status(
        "Created",
        format!("{} `{}` package", opts.kind, package_name),
    )?;
    Ok(())
}
