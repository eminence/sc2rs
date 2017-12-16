use super::UnitID;
use super::types;

pub fn is_worker(unit_id: u32) -> bool {
    unit_id == UnitID::SCV as u32 || unit_id == UnitID::Drone as u32 || unit_id == UnitID::Probe as u32
}

pub fn is_command_base(id: u32) -> bool {
    id == UnitID::CommandCenter as u32 ||
        id == UnitID::PlanetaryFortress as u32 ||
        id == UnitID::OrbitalCommand as u32
}

pub fn get_nearest_unit<'a, I>(source: types::Point, list: I) -> Option<&'a types::Unit> where
    I: Iterator<Item=&'a types::Unit> {
    list
        .map(|u| (u, u.pos.distance_between(&source)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|pair| pair.0)
}