use super::UnitIDs;

pub fn is_worker(unit_id: UnitIDs) -> bool {
    unit_id == UnitIDs::SCV || unit_id == UnitIDs::Drone || unit_id == UnitIDs::Probe
}

pub fn is_mineral_field(id: UnitIDs) -> bool {
    id == UnitIDs::MineralField || id == UnitIDs::MineralField750
}

