use log::trace;

/// Return the percentage charge of all batteries in a device.  
/// Returns None if there are no batteries.
pub fn battery_percentage() -> battery::Result<Option<u32>> {
    let manager = battery::Manager::new()?;

    // Adding up the charges and charge capacities of the batteries, not their percentages.
    let mut total_capacity = 0.0;
    let mut total_charge = 0.0;

    for (idx, maybe_battery) in manager.batteries()?.enumerate() {
        let battery = maybe_battery?;
        trace!("Battery #{idx} charge: {}", battery.energy().value);
        trace!("Battery #{idx} capacity: {}", battery.energy_full().value);
        total_capacity += battery.energy_full().value;
        total_charge += battery.energy().value;
    }

    if total_capacity == 0.0 && total_charge == 0.0 {
        trace!("No batteries found");
        return Ok(None);
    }

    trace!("Total charge of all batteries: {total_charge}");
    trace!("Total capacity of all batteries: {total_capacity}");

    let average_percentage = (total_charge / total_capacity) * 100.0;
    Ok(Some(average_percentage as u32))
}

/// Returns true if at least one battery is charging.
pub fn is_charging() -> battery::Result<bool> {
    let manager = battery::Manager::new()?;

    for (idx, maybe_battery) in manager.batteries()?.enumerate() {
        let battery = maybe_battery?;
        if battery.state() == battery::State::Charging {
            trace!("Battery #{idx} is charging.");
            return Ok(true);
        }
        trace!("Battery #{idx} is not charging.");
    }

    Ok(false)
}