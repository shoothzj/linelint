use clap::Command;
use linelint::config::Config;
use linelint::line::LineEnding;
use linelint::linter::Linter;
use linelint::rule::line_end_lint::LineEndLint;
use linelint::rule::trailing_ws_lint::TrailingWhitespaceLint;
use std::path::Path;

fn check(current_dir: &Path, linter: &Linter) {
    match linter.check_files_in_dir(current_dir) {
        Ok(issues) => {
            if issues.is_empty() {
                println!("No issues found.");
            } else {
                for issue in issues {
                    println!(
                        "{}: {} in file {} at line {}",
                        issue.rule, issue.description, issue.filename, issue.line_number
                    );
                }
                std::process::exit(2);
            }
        }
        Err(errors) => {
            for e in errors {
                eprintln!("Error checking files: {:?}", e);
            }
        }
    }
}

fn format(current_dir: &Path, linter: &Linter) {
    match linter.format_files_in_dir(current_dir) {
        Ok(_) => println!("Files formatted successfully."),
        Err(errors) => {
            for e in errors {
                eprintln!("Error formatting files: {:?}", e);
            }
        }
    }
}

fn main() {
    let matches = Command::new("linelint-cli")
        .version("0.0.1")
        .about("A command-line tool for linting and fixing line formatting issues")
        .subcommand(Command::new("check").about("Check files for lint issues"))
        .subcommand(Command::new("format").about("Automatically format files to fix lint issues"))
        .get_matches();

    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    let mut config = Config::new(LineEnding::Auto);
    config.add_rule(Box::new(LineEndLint {}));
    config.add_rule(Box::new(TrailingWhitespaceLint {}));

    let linter = Linter::new(&config);

    if matches.subcommand().is_none() {
        println!("No subcommand provided, defaulting to 'check'...");
        check(&current_dir, &linter);
    } else if matches.subcommand_matches("check").is_some() {
        check(&current_dir, &linter);
    } else if matches.subcommand_matches("format").is_some() {
        format(&current_dir, &linter);
    }
}
