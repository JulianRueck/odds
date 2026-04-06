use clap::Parser;

pub const BASH_ZSH_SCRIPT: &str = r#"
o() {
    if [ "$#" -eq 0 ]; then
        cd ~
    elif [ "$#" -eq 1 ] && [ "$1" = "-" ]; then
        cd -
    else
        local result
        result=$(command odds "$@") && [ -n "$result" ] && cd "$result"
    fi
}
odds() { o "$@" }
"#;

#[derive(Parser)]
#[command(name = "o")]
pub struct Cli {
    #[arg(long)]
    pub init: Option<String>,
    #[arg(long)]
    pub seed: bool,
    pub tokens: Vec<String>,
}

impl Cli {
    pub fn handle_init(&self) -> bool {
        if let Some(shell_type) = &self.init {
            match shell_type.as_str() {
                "bash" | "zsh" => println!("{}", BASH_ZSH_SCRIPT),
                _ => eprintln!("Unsupported shell: {}", shell_type),
            }
            return true;
        }
        false
    }
}
