use std::collections::HashMap;
use once_cell::sync::Lazy;

mod load_res;
mod models;
mod requests;

pub use models::api::{Status, WarInfo};
pub use models::Planet;
pub use requests::{get_status, get_war_info};


pub const BASE_URL: &str = "https://api.live.prod.thehelldiversgame.com/api";

pub static PLANETS: Lazy<HashMap<i64, Planet>> = Lazy::new(load_res::load_planets);
pub static FACTIONS: Lazy<HashMap<i64, models::Faction>> = Lazy::new(load_res::load_factions);
pub static SECTORS: Lazy<HashMap<i64, models::Sector>> = Lazy::new(load_res::load_sectors);


/// Get the name of a planet
///
/// Arguments:
///   id: i64 - The ID of the planet
pub fn get_planet_name(id: i64) -> String {
    match PLANETS.get(&id) {
        Some(planet) => planet.name.clone(),
        None => format!("Unknown Planet {}", id),
    }
}

/// Get the name of a faction
///
/// Arguments:
///    id: i64 - The ID of the faction
pub fn get_faction_name(id: i64) -> String {
    match FACTIONS.get(&id) {
        Some(faction) => faction.name.clone(),
        None => format!("Unknown Faction {}", id),
    }
}

/// Get the name of a sector
///
/// Arguments:
///   id: i64 - The ID of the sector
pub fn get_sector_name(id: i64) -> String {
    match SECTORS.get(&id) {
        Some(sector) => sector.name.clone(),
        None => format!("Unknown Sector {}", id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_status() {
        let _status = match get_status(801) {
            Ok(status) => status,
            Err(e) => panic!("Error: {}", e),
        };
    }

    #[test]
    fn test_get_war_info() {
        let _war_info = match get_war_info(801) {
            Ok(war_info) => war_info,
            Err(e) => panic!("Error: {}", e),
        };
    }

    #[test]
    fn test_load_planets() {
        assert!(PLANETS.len() > 0);
    }

    #[test]
    fn test_load_factions() {
        assert!(FACTIONS.len() > 0);
    }

    #[test]
    fn test_load_sectors() {
        assert!(SECTORS.len() > 0);
    }

    #[test]
    fn test_get_planet_name() {
        assert_eq!(get_planet_name(0), "Super Earth");
    }

    #[test]
    fn test_get_faction_name() {
        assert_eq!(get_faction_name(1), "Humans");
    }

    #[test]
    fn test_get_sector_name() {
        assert_eq!(get_sector_name(0), "Sol");
    }

    #[test]
    fn test_war_info_planet_name() {
        let war_info = match get_war_info(801) {
            Ok(war_info) => war_info,
            Err(e) => panic!("Error: {}", e),
        };
        for planet_info in war_info.planet_infos {
            assert!(!planet_info.planet_name.is_empty())
        }
    }

    #[test]
    fn test_status_planet_name() {
        let status = match get_status(801) {
            Ok(status) => status,
            Err(e) => panic!("Error: {}", e),
        };
        for planet_status in status.planet_status {
            assert!(!planet_status.planet_name.is_empty())
        }
    }

}