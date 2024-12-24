mod clouds;
mod power;
mod xplanet;

use xplanet::{create_xplanet_config, generate_earth_map};
use clouds::download_clouds;
use power::{battery_percentage, is_charging};
use std::{fs, io, path::Path, time::{Duration, SystemTime}};
use log::{debug, error, info, log_enabled, trace, warn};

// Directories
const CONFIG_FILENAME: &str = "down2earth.config";
const CLOUDS_FILENAME: &str = "clouds.png";
const OUTPUT_FILENAME: &str = "down2earth.png";

const ZOOM_LEVEL: u32 = 2;

pub enum Down2EarthError {
    PowerStatusExit,
    CloudDownloadError,
    XPlanetError,
    SetWallpaperError,
}

fn is_file_older_than<P: AsRef<Path>>(file: P, duration: Duration) -> io::Result<bool> {
    let last_modified = fs::metadata(file)?.modified()?;
    let now = SystemTime::now();
    let elapsed = now.duration_since(last_modified).expect("File was modified in the past");
    Ok(elapsed > duration)
}

pub fn down2earth(
        geometry: (u32, u32), 
        working_directory: &str, 
        min_battery_percentage: u32, 
        clouds_lifespan: Duration, 
        clouds_api_key: Option<String>,
        force: bool
    ) -> Result<(), Down2EarthError> {
    if let Some(ref api_key) = clouds_api_key {
        if log_enabled!(log::Level::Trace) {
            warn!("Enabling trace level logging exposes your API Key! Be careful who you share these logs with.");
            trace!("API Key = {api_key}");
        }
    } else {
        warn!("No API key set!  Without one, your IP address will be limited to 500 megapixels of cloud downloads per day.");
        warn!("Get an access key at https://realearth.ssec.wisc.edu/users/ to increase the threshold to 1000 megapixels and avoid watermarks.");
    }

    let config_path = format!("{working_directory}/{CONFIG_FILENAME}");
    let clouds_path = format!("{working_directory}/{CLOUDS_FILENAME}");
    let output_path = format!("{working_directory}/{OUTPUT_FILENAME}");

    match (force, battery_percentage().unwrap()) {
        (true, _) => info!("Force flag set, ignoring battery status."),
        (false, Some(percentage)) if percentage > min_battery_percentage => {
            info!("Battery is above minimum charge, continuing.");
        },
        (false, Some(_)) if is_charging().unwrap() => {
            info!("Battery is below minimum charge but charging, continiuing.");
        },
        (false, Some(_)) => {
            error!("Battery is below minimum charge and is not charging, exiting.");
            return Err(Down2EarthError::PowerStatusExit);
        }
        (false, None) => info!("No batteries found, assuming AC power and continuing."),
    }

    if !Path::new(&config_path).exists() {
        info!("Xplanet config file not found, creating new one at '{config_path}'.");
        create_xplanet_config(&config_path, &clouds_path).unwrap();
    } else {
        debug!("Xplanet config file found at '{config_path}'.");
    }

    if !Path::new(&clouds_path).exists() || is_file_older_than(&clouds_path, clouds_lifespan).unwrap() {
        info!("Cloud map not found or is stale, downloading new one.");
        if let Err(error) = download_clouds(ZOOM_LEVEL, &clouds_path, clouds_api_key) {
            debug!("Error getting cloud map: {:?}", error);
            return Err(Down2EarthError::CloudDownloadError);
        }
    } else {
        debug!("Cloud map exists and is not stale, skipping download.");
    }

    info!("Generating planet image.");
    if let Err(error) = generate_earth_map(geometry, &config_path, &output_path) {
        debug!("Error generating planet image: {:?}", error);
        return Err(Down2EarthError::XPlanetError);
    }

    info!("Setting wallpaper.");
    if wallpaper::set_from_path(&output_path).is_err() {
        return Err(Down2EarthError::SetWallpaperError);
    }

    Ok(())
}