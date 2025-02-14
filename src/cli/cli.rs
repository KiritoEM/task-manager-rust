use clap::{command, Arg, ColorChoice, Command};

//{n}Salama

pub fn cli_builder () -> Command {
    command!()
    .about("\n\nTask manager with Rust")
    .author("KiritoEM")
    .color(ColorChoice::Always)
    .arg(
        Arg::new("config")
        .short('c')
        .value_name("FILE")
        .help("Provide a config to my profile file")
    
    )
    .arg_required_else_help( true )
}