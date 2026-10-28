use eery::{
    config::{
        Config,
        tools::{parse_tools, run_tools},
    },
    parse::parse_speech,
    setup,
    voice::transcribe_audio,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    setup()?;
    let config = Config::get_args().unwrap();
    let tools = parse_tools(Some(config.tools_path.clone()))?;

    // I was working on having a full running daemon and listening for a wakeword to trigger record
    // and parse but tapping a button to listen was more intuitive and saved me a lot of time
    // so here goes.

    // println!("Listening..");
    // let recorded = record()?;
    // println!("Parsing..");
    // if !recorded {
    //     eprintln!("Nothing recorded");
    //     return Ok(());
    // }

    let output = transcribe_audio(&config)?;
    let output = output.text;
    let data = parse_speech(&config, &output)?;
    if data.confidence <= 0.3 {
        eprintln!("Confidence too low. Aborting.");
        return Ok(());
    }
    println!("data: {:#?}", data);

    run_tools(&tools, &data.function_calls)?;

    Ok(())
}
