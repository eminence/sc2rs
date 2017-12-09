use super::UnitIDs;
use super::types;

pub fn is_worker(unit_id: u32) -> bool {
    unit_id == UnitIDs::SCV as u32 || unit_id == UnitIDs::Drone as u32 || unit_id == UnitIDs::Probe as u32
}

pub fn is_command_base(id: u32) -> bool {
    id == UnitIDs::CommandCenter as u32 ||
        id == UnitIDs::PlanetaryFortress as u32 ||
        id == UnitIDs::OrbitalCommand as u32
}

pub fn get_nearest_unit<'a, I>(source: types::Point, list: I) -> Option<&'a types::Unit> where
    I: Iterator<Item=&'a types::Unit> {
    list
        .map(|u| (u, u.pos.distance_between(&source)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|pair| pair.0)
}