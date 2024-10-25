mod config;
mod t0;

use config::Config;
use std::option;
use t0::T0;

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
    let pattern = std::env::args().nth(1);
    match pattern.as_deref() {
        None => println!("display???"),
        Some("--") | Some("-a") => println!("add"),
        Some("-h") => println!("{}", usage),
        _ => println!("Invalid Input\n{}", usage),
    }
}
