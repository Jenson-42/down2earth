use std::{path::Path, io::{Result, Write}, process::{Command, ExitStatus}, fs::File};
use log::trace;

/// Where to find the xplanet command.
const XPLANET_COMMAND: &str = "/usr/bin/xplanet";

pub fn create_xplanet_config<P: AsRef<Path>>(path: P, cloudmap_path: P) -> Result<()> {
    let mut output = File::create(path)?;
    writeln!(output, "[earth]")?;
    // writeln!(output, "map={EARTHNOW_DIR}/earthnow.jpg")?;
    // writeln!(output, "night_map={EARTHNOW_DIR}/earthnow_night.jpg")?;
    writeln!(output, "cloud_map={}", cloudmap_path.as_ref().display())?;
    Ok(())
}

/// Run the xplanet command.
pub fn generate_earth_map<P: AsRef<Path>>(geometry: (u32, u32), config: P, output: P) -> Result<ExitStatus> {
    let mut xplanet_command = Command::new(XPLANET_COMMAND);
    xplanet_command
        .args(["-num_times", "1"])
        .args(["-output", output.as_ref().as_os_str().to_string_lossy().as_ref()])
        .args(["-geometry", &format!("{}x{}", geometry.0, geometry.1)])
        .args(["-body", "earth"])
        .args(["-projection", "mercator"])
        .args(["-proj_param", "72"])
        .args(["-config", config.as_ref().as_os_str().to_string_lossy().as_ref()]);

    trace!("Running xplanet command: {:?}", xplanet_command);

    xplanet_command
        .spawn()?
        .wait()
}
