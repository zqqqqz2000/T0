mod config;
mod t0;

use config::Config;
use regex::Regex;
use std::fs::File;
use t0::{Todo, T0};

fn parse_todo(input: &str) -> Todo {
    // TODO: first parse xxx(xxx),xxx(xxx), then check the first -> end valid range to get the
    // content, then parse
    let extra_options_re =
        Regex::new(r"(?P<content>.+?)(?:\s+ddl\((?P<ddl>.+?)\))?(?:\s+tags?\((?P<tags>.+?)\))?$")
            .unwrap();
    let caps = extra_options_re.captures(input).unwrap();
    let tags = caps["tags"]
        .split(",")
        .map(|tag| tag.trim().to_string())
        .filter(|each| !each.is_empty())
        .collect();

    Todo {
        id: 0,
        content: caps["content"].to_string(),
        description: None,
        tags,
        ddl: None,
        completed: false,
        created_at: 0,
        completed_at: None,
    }
}

fn args_after_nth(nt: usize) -> String {
    std::env::args().skip(nt).collect::<Vec<String>>().join(" ")
}

fn main() {
    let usage = "Usage: T0 [options] \
    \n  no options: Display todo list \
    \n  -- <todo>, -a <todo>, --add <todo>: Add todo list \
    \n    Add todo list options:
    \n    <end of todo> tag[s](<tag1>, <tag2>, <tag3>...): Add tags to todo list \
    \n    <end of todo> ddl(<date>): Add deadline to todo list \
    \n    <end of todo> ddl(<offset>): Add offset to current deadline to todo list, offset: +|-n y[ear]|m[onth]|d[ay]|h[our]|M[inute]|S[esonds] \
    \n  pop: Pop todo list \
    \n  -h, --help: Show this help message \
    \n  -l, --list: List todo list \
    \n  -d <id>, --delete <id>: Delete todo list \
    \n  -c <id>, --complete <id>: Complete todo list \
    \n  -u <id>, --uncomplete <id>: Uncomplete todo list \
    \n  -r, --reindex: Reindex todo list \
    ";
    let default_config = serde_json::from_str::<Config>(include_str!("./default.json")).unwrap();
    let pattern = std::env::args().nth(1);

    let config = File::open("~/.config/t0/config.json")
        .map(|file| serde_json::from_reader::<_, Config>(file).expect("Invalid config file"))
        .unwrap_or_else(|_| {
            println!("No config file found, using default config");
            default_config
        });

    let mut t0 = T0::new(config);

    match pattern.as_deref() {
        None => t0.display(),
        Some("--") | Some("-a") => {
            let raw_todo = args_after_nth(2);
            let todo = parse_todo(raw_todo.as_str());
            t0.add(todo);
        }
        Some("-h") => println!("{}", usage),
        _ => println!("Invalid Input\n{}", usage),
    }
}
