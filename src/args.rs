use clap::{Parser, Subcommand};

pub const BASH_ZSH_SCRIPT: &str = r#"
o() {
    local result
    result=$(command odds query "$@")
    if [ -n "$result" ]; then
        cd "$result"
    elif [ "$#" -eq 1 ] && [ "$1" = "-" ]; then
        cd -
    fi
}
"#;

pub const BASH_EXTRA: &str = r#"
_odds_register() {
    if [ "$PWD" != "$_ODDS_LAST_PWD" ]; then
        (command odds register --pwd "$PWD" &>/dev/null &)
        _ODDS_LAST_PWD="$PWD"
    fi
}
PROMPT_COMMAND="_odds_register;${PROMPT_COMMAND}"
"#;

pub const ZSH_EXTRA: &str = r#"
chpwd() {
    (command odds register --pwd "$PWD" &>/dev/null &)
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
    Register {
        #[arg(long)]
        pwd: String,
    },

    #[command(hide = true)]
    Query {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        tokens: Vec<String>,
    },
}

impl Cli {
    pub fn handle_init(shell_type: &str) -> bool {
        match shell_type {
            "bash" => {
                print!("{}{}", BASH_ZSH_SCRIPT, BASH_EXTRA);
                true
            }
            "zsh" => {
                print!("{}{}", BASH_ZSH_SCRIPT, ZSH_EXTRA);
                true
            }
            _ => {
                eprintln!("Unsupported shell: {}", shell_type);
                false
            }
        }
    }
}
