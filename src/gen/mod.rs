extern crate serde;
extern crate serde_json;
// This is the only non-generated file in this folder

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::hash::Hash;
use std::cmp::Eq;

mod units;
mod abilities;

pub use self::units::*;
pub use self::abilities::*;

use super::types;
use super::types::FromU32;

fn load_data<T, P, V, F>(p: P, get_id: F) -> HashMap<T, V>
where
    T: FromU32 + Hash + Eq,
    P: AsRef<Path>,
    V: serde::de::DeserializeOwned,
    F: Fn(&V) -> u32 + Sized,
{
    let root = Path::new(file!()).parent().unwrap();
    let file = File::open(root.join(p)).unwrap();
    let data: Vec<V> = serde_json::from_reader(file).unwrap();

    let mut m = HashMap::new();
    for data in data.into_iter() {
        let key = FromU32::from_u32(get_id(&data)).unwrap();
        m.insert(key, data);
    }
    m
}


lazy_static! { // sad face that rust can't figure out the type of d in the below closures
    pub static ref UNIT_DATA: HashMap<UnitIDs, types::UnitTypeData> = {load_data("units.json", |d: &types::UnitTypeData| d.unit_id)};
    pub static ref ABILITY_DATA: HashMap<AbilityIDs, types::AbilityData> = {load_data("abilities.json", |d: &types::AbilityData| d.ability_id)};
}


impl UnitIDs {
    pub fn is_worker(&self) -> bool {
        super::utils::is_worker(*self as u32)
    }

    pub fn is_mineral_field(&self) -> bool {
        super::utils::is_mineral_field(*self as u32)
    }
    pub fn is_command_base(&self) -> bool {
        super::utils::is_command_base(*self as u32)
    }

}