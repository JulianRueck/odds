use clap::{Parser, Subcommand};

pub const BASH_ZSH_SCRIPT: &str = r#"
o() {
    if [ "$#" -eq 0 ]; then
        cd ~
    elif [ "$#" -eq 1 ] && [ "$1" = "-" ]; then
        cd -
    else
        local result
        result=$(command odds query "$@") && [ -n "$result" ] && cd "$result"
    fi
}
"#;

#[derive(Parser)]
#[command(name = "odds")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        shell: String,
    },

    Seed,

    #[command(hide = true)]
    Query {
        tokens: Vec<String>,
    },
}

impl Cli {
    pub fn handle_init(shell_type: &str) -> bool {
        match shell_type {
            "bash" | "zsh" => {
                // Use print! to avoid trailing double-newlines in eval
                print!("{}", BASH_ZSH_SCRIPT);
                true
            }
            _ => {
                eprintln!("Unsupported shell: {}", shell_type);
                false
            }
        }
    }
}
