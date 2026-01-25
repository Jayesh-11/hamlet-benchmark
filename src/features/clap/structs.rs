use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Italics
    #[arg(short, long, default_value_t = true)]
    pub italics: bool,

    /// Delay in ms on each iteration
    #[arg(short, long, default_value_t = 0)]
    pub delay: u64,

    /// Enable random colors for each iteration
    #[arg(short, long, default_value_t = false)]
    pub random_color_mode: bool,

    /// Just print, no randomness
    #[arg(short, long, default_value_t = false)]
    pub just_print: bool,

    /// Benchmark mode, no prints
    #[arg(short, long, default_value_t = false)]
    pub benchmark: bool,
}
