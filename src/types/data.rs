
use sc2_protobuf::protos;
use failure;
use super::{ToProtobuf, FromProtobuf};

use super::common::*;

#[derive(Debug, Clone, FromProtobuf, Serialize, Deserialize)]
pub struct AbilityData {
    /// Stable ID
    #[Get]
    pub ability_id: u32,

    /// Catalog name of the ability
    pub link_name: String,

    /// Catalog index of the ability
    #[Get]
    pub link_index: u32,

    /// Name used for the command card
    pub button_name: String,

    /// A human friendly name when the button name or link name isn't descriptive
    pub friendly_name: Option<String>,

    /// Hotkey
    pub hotkey: Option<String>,

    /// The ability id may be represented by the given more generic id
    #[Get]
    pub remaps_to_ability_id: u32,

    #[Get]
    pub target: AbilityData_Target,

    /// If true, the ability may be used on this set of mods/map
    #[Get]
    pub available: bool,

    #[Get]
    pub allow_minimap: bool,
    #[Get]
    pub allow_autocast: bool,
    #[Get]
    pub is_building: bool,
    #[Get]
    pub footprint_radius: f32,
    #[Get]
    pub is_instant_placement: bool,
    #[Get]
    pub cast_range: f32,
}



#[derive(Clone, PartialEq, Eq, Debug, Hash, FromProtobuf, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum AbilityData_Target {
    None = 1,
    Point = 2,
    Unit = 3,
    PointOrUnit = 4,
    PointOrNone = 5,
}


#[derive(Clone, PartialEq, Eq, Debug, Hash, FromProtobuf, Serialize, Deserialize)]
pub enum Attribute {
    Light = 1,
    Armored = 2,
    Biological = 3,
    Mechanical = 4,
    Robotic = 5,
    Psionic = 6,
    Massive = 7,
    Structure = 8,
    Hover = 9,
    Heroic = 10,
    Summoned = 11,
}


#[derive(Debug, Clone, FromProtobuf, Serialize, Deserialize)]
pub struct DamageBonus {
    #[Get]
    pub attribute: Attribute,
    #[Get]
    pub bonus: f32,
}


#[derive(Clone, PartialEq, Eq, Debug, Hash, FromProtobuf, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Weapon_TargetType {
    Ground = 1,
    Air = 2,
    Any = 3,
}

#[derive(Debug, Clone, FromProtobuf, Serialize, Deserialize)]
pub struct Weapon {
    #[Get]
    pub field_type: Weapon_TargetType,
    #[Get]
    pub damage: f32,
    pub damage_bonus: Vec<DamageBonus>,
    /// Number of hits per attack
    ///
    /// e.g. Colossus has 2 beams
    #[Get]
    pub attacks: u32,
    #[Get]
    pub range: f32,
    #[Get]
    pub speed: f32,
}




#[derive(Debug, Clone, FromProtobuf, Serialize, Deserialize)]
pub struct UnitTypeData {
    /// Stable ID
    #[Get]
    pub unit_id: u32,
    /// Catalog name of the unit
    pub name: String,
    #[Get]
    pub available: bool,
    #[Get]
    pub cargo_size: u32,
    #[Get]
    pub mineral_cost: u32,
    #[Get]
    pub vespene_cost: u32,
    #[Get]
    pub food_required: f32,
    #[Get]
    pub food_provided: f32,
    /// The ability that builds this unit
    #[Get]
    pub ability_id: u32,
    #[Get]
    pub race: Race,
    #[Get]
    pub build_time: f32,
    #[Get]
    pub has_vespene: bool,
    #[Get]
    pub has_minerals: bool,
    /// Range unit reveals vision
    #[Get]
    pub sight_range: f32,
    /// Other units that satisfy the same tech requirement
    pub tech_alias: Vec<u32>,
    /// The morphed variant of this unit
    #[Get]
    pub unit_alias: u32,
    /// Structure required to build this unit (or any with the same `tech_alias`
    #[Get]
    pub tech_requirement: u32,
    #[Get]
    pub require_attached: bool,

    pub attributes: Vec<Attribute>,
    #[Get]
    pub movement_speed: f32,
    #[Get]
    pub armor: f32,
    pub weapons: Vec<Weapon>,
}



#[derive(Debug, FromProtobuf)]
pub struct UpgradeData {
    #[Get]
    pub upgrade_id: u32,
    pub name: String,
    #[Get]
    pub mineral_cost: u32,
    #[Get]
    pub vespene_cost: u32,
    #[Get]
    pub research_time: f32,
    #[Get]
    pub ability_id: u32,
}

#[derive(Debug, FromProtobuf)]
pub struct BuffData {
    #[Get]
    pub buff_id: u32,
    pub name: String,
}

#[derive(Debug, FromProtobuf)]
pub struct EffectData {
    #[Get]
    pub effect_id: u32,
    pub name: String,
    pub friendly_name: String,
    #[Get]
    pub radius: f32,
}
