//https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/
//https://rust-cli.github.io/book/index.html
use clap::{Arg, Parser};


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    fn main() {
        let matches = App::new("My CLI App")
            .version("1.0")
            .author("Your Name")
            .about("An example CLI application with custom flags")
            .arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .value_name("FILE")
                    .help("Sets the input file to use")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("FILE")
                    .help("Sets the output file to use")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .help("Sets the level of verbosity"),
            )
            .get_matches();

        // Accessing the flag values based on user input
        if let Some(input_file) = matches.value_of("input") {
            println!("Input file specified: {}", input_file);
        }

        if let Some(output_file) = matches.value_of("output") {
            println!("Output file specified: {}", output_file);
        }

        if matches.is_present("verbose") {
            println!("Verbose mode is enabled");
        }
}
