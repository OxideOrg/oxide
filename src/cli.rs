use clap::{Parser, command};

const ABOUT: &str = "TODO";

#[derive(Parser, Debug, Clone, Default)]
#[command(
  about = ABOUT,
  long_about = ABOUT,
)]
/// Command line options.
pub struct CliOpt {
    #[arg(short = 'V', long = "version", help = "Print version")]
    version: bool,

    #[arg(help = "Edit file(s)")]
    file: Vec<String>,
}

impl CliOpt {
    /// Input files.
    pub fn file(&self) -> &Vec<String> {
        &self.file
    }

    /// Version.
    pub fn version(&self) -> bool {
        self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_opt_should_parse_when_valid_arguments() {
        let input = [
            vec!["oxide".to_string()],
            vec!["oxide".to_string(), "--version".to_string()],
            vec!["oxide".to_string(), "README.md".to_string()],
        ];

        let expect = [
            CliOpt {
                file: vec![],
                version: false,
            },
            CliOpt {
                file: vec![],
                version: true,
            },
            CliOpt {
                file: vec!["README.md".to_string()],
                version: false,
            },
        ];

        assert_eq!(input.len(), expect.len());
        let n = input.len();
        for i in 0..n {
            let actual = CliOpt::parse_from(&input[i]);
            assert_eq!(actual.file, expect[i].file);
            assert_eq!(actual.version(), expect[i].version());
        }
    }
}
