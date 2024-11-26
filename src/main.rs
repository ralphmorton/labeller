use clap::Parser;

use labeller::app::App;
use labeller::example::{Example, LabelledExample};

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    prelabelled: bool,

    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long)]
    label: Vec<String>
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.label.is_empty() {
        println!("No labels provided, exiting...");
        std::process::exit(0);
    }

    let examples = load_examples(&args.input, args.prelabelled)?;
    if examples.is_empty() {
        println!("No examples provided, exiting...");
        std::process::exit(0);
    }

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut app = App::new(examples, args.label).unwrap();
    app.run(&mut terminal)?;

    write_labelled_examples(&args.output, &app.examples)?;

    ratatui::restore();

    Ok(())
}

fn load_examples(input_path: &str, prelabelled: bool) -> anyhow::Result<Vec<LabelledExample>> {
    let text = std::fs::read_to_string(input_path)?;

    let examples = if prelabelled {
        let examples = serde_json::from_str(&text)?;
        examples
    } else {
        let examples: Vec<Example> = serde_json::from_str(&text)?;
        examples.into_iter()
            .map(|example| LabelledExample { example, label: None })
            .collect()
    };

    Ok(examples)
}

fn write_labelled_examples(
    output_path: &str,
    examples: &Vec<LabelledExample>
) -> anyhow::Result<()> {
    let json = serde_json::to_string(examples)?;
    std::fs::write(output_path, json)?;

    Ok(())
}
