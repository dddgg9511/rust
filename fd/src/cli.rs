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
    #[arg(
        default_value = "",
        hide_default_value = true,
        value_name = "pattern",
        help = "the search pattern (a regular expression, unless '--glob' is used; optional)",
        long_help
    )]
    pub pattern: String,

}

impl Opts {

    pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
        // would it make sense to concatenate these?
        let paths = if !self.path.is_empty() {
            &self.path
        } else if !self.search_path.is_empty() {
            &self.search_path
        } else {
            let current_directory = Path::new(".");
            ensure_current_directory_exists(current_directory)?;
            return Ok(vec![self.normalize_path(current_directory)]);
        };
        Ok(paths
            .iter()
            .filter_map(|path| {
                if filesystem::is_existing_directory(path) {
                    Some(self.normalize_path(path))
                } else {
                    print_error(format!(
                        "Search path '{}' is not a directory.",
                        path.to_string_lossy()
                    ));
                    None
                }
            })
            .collect())
    }

}
