use clap::{
    error::ErrorKind, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command, Parser,
    ValueEnum,
};
#[cfg(feature = "completions")]
use clap_complete::Shell;
#[derive(Parser)]
#[command(
name = "fd",
max_term_width = 98,
args_override_self = true,
group(ArgGroup::new("execs").args(&["exec", "exec_batch", "list_details"]).conflicts_with_all(&[
"max_results", "has_results", "count"])),
)]
pub struct Opts {
}

impl Opts {
}