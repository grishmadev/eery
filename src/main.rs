use std::error::Error;

use eery::{
    config::tools::Tool,
    parse::{parse_command, parse_tools},
    voice::{record, transcribe_audio},
};

fn main() -> Result<(), Box<dyn Error>> {
    record()?;
    let output = transcribe_audio()?;
    let output = output.text;
    let data = parse_command(&output)?;
    println!("data: {:#?}", data);

    let tools = Tool::parse(None)?;

    parse_tools(data.function_calls)?;

    Ok(())
}
