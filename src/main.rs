use anyhow::Result;
use candid_parser::bindings::{javascript, typescript};
use candid_parser::pretty;
use candid_parser::pretty_check_file;
use clap::Parser;
use std::fs::{create_dir_all, write};
use std::path::Path;
use std::path::PathBuf;

/// The arguments for the command-line
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Path to the input `.did` file.
    #[clap(short, long)]
    input: PathBuf,

    /// Optional output file path. If not provided, the output will be printed to stdout.
    #[clap(short, long)]
    output: Option<PathBuf>,

    /// Target output format: either `js` for JavaScript, `ts` for TypeScript or `did` for Candid.
    #[clap(short, long, value_parser = ["js", "ts", "did"])]
    target: String,
}

/// Writes the compiled output to the specified file, ensuring that any missing directories
/// in the file path are created.
///
/// # Arguments
///
/// * `output_path` - A reference to the path where the output file will be written.
/// * `content` - The content to be written to the file.
///
/// # Errors
///
/// This function will return an error if the directories cannot be created or if the file
/// cannot be written.
fn write_output(output_path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = output_path.parent() {
        create_dir_all(parent)?;
    }

    write(output_path, content)?;

    Ok(())
}

/// A function to generate JavaScript, TypeScript, or DID content from a `.did` file.
///
/// This `main` function serves as a wrapper around the `didc` tool. It parses command-line arguments,
/// processes the input `.did` file, and outputs the result either to stdout or to a specified file,
/// depending on user input.
///
/// The parser resolves any imported services and produces a single, self-contained output file
/// — whether it's JavaScript, TypeScript, or a `.did` definition.
///
/// # Errors
///
/// Returns an error if the input `.did` file cannot be processed, or if writing to the output file (when specified) fails.
fn main() -> Result<()> {
    let cli = Cli::parse();

    let (env, actor, prog) = pretty_check_file(&cli.input)?;

    let content = match cli.target.as_str() {
        "ts" => typescript::compile(&env, &actor, &prog),
        "js" => javascript::compile(&env, &actor),
        "did" => pretty::candid::compile(&env, &actor),
        _ => unreachable!(),
    };

    match cli.output {
        None => println!("{content}"),
        Some(output) => write_output(&output, &content)?,
    }

    Ok(())
}
