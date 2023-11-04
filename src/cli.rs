use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Should rendering be single or multi threaded?
    #[arg(long, short, default_value = "true")]
    pub threaded: bool,

    /// Outputted image width
    #[arg(long, short)]
    pub width: u32,

    /// Outputted image height
    #[arg(long, short)]
    pub height: u32,

    /// Proximity tolerance to count a ray collision
    #[arg(long, short, default_value = "0.0003")]
    pub collision_tolerance: f32,
}
