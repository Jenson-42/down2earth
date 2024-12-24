use std::process::ExitCode;

use clap::Parser;
use down2earth::down2earth;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Wallpaper height.  Match this to your display's height.
    #[arg(long)]
    height: u32,

    /// Wallpaper width.  Match this to your display's width.
    #[arg(long)]
    width: u32,

    /// Working directory for earthnow temp files and config.
    #[arg(long)]
    dir: String,

    /// Realearth API key for realtime cloud maps. 
    /// Go to https://realearth.ssec.wisc.edu/users/ to set up an account and get your access key.
    /// Without one of these keys, your IP address will be limited to 500 megapixels per day before a watermark is applied.
    #[arg(long)]
    api_key: Option<String>,

    /// Minimum battery percentage for the script to continue running.
    #[arg(long, default_value_t = 20)]
    min_battery_percentage: u32,

    /// Re-download and stitch cloud maps if they are older than this duration in seconds.
    #[arg(long, default_value_t = 10800)]
    cloud_lifespan_seconds: u64,

    /// Force the map to generate even if the battery is below the threshold.
    /// In future, if this program detects network type and availability, this flag will cause it to skip those checks too.
    #[arg(long, short, default_value_t = false)]
    force: bool
}

fn main() -> ExitCode {
    env_logger::init();
    let args = Args::parse();
    
    match down2earth(
        (args.width, args.height), 
        &args.dir, 
        args.min_battery_percentage, 
        std::time::Duration::from_secs(args.cloud_lifespan_seconds), 
        args.api_key,
        args.force
    ) {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
