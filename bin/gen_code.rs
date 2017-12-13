extern crate sc2;
extern crate serde;
extern crate serde_json;
//extern crate quote;


use sc2::types;

use std::fs::File;
use std::io::Write;
use std::path::Path;

// This utility generates rust code used by the library:
// * The protobuf code (see the sc2-protobuf sub-crate)
// * The ID lookup tables

fn main() {
    let coord = sc2::Coordinator::new();
    let coord = coord.launch().expect("Failed to launch game");
    let req = types::RequestCreateGame {
        map: types::RequestMap::LocalMap(types::LocalMap {
            MapPath: "Z:\\devel\\Melee\\Simple64.SC2Map".to_owned(),
        }),
        player_setup: vec![
            types::PlayerSetup {
                field_type: types::PlayerType::Computer,
                race: types::Race::Terran,
                difficulty: types::Difficulty::Easy,
            },
            types::PlayerSetup {
                field_type: types::PlayerType::Computer,
                race: types::Race::Zerg,
                difficulty: types::Difficulty::Easy,
            },
            types::PlayerSetup {
                field_type: types::PlayerType::Computer,
                race: types::Race::Protoss,
                difficulty: types::Difficulty::Easy,
            },
            types::PlayerSetup {
                field_type: types::PlayerType::Observer,
                race: types::Race::Protoss,
                difficulty: types::Difficulty::Easy,
            }
        ],
        disable_fog: false,
        random_seed: None,
        realtime: false,
    };


    let coord = coord.create_game(req).unwrap();

    let req = types::RequestJoinGame {
        participation: types::Participation::ObservedPlayerId(1),
        options: types::InterfaceOptions {
            raw: false,
            score: false,
            feature_layer: None,
        },
    };

    let mut coord = coord.join_game(req).unwrap();


    let data: types::ResponseData = coord.game_data(types::RequestData {
        ability_id: true,
        buff_id: true,
        effect_id: true,
        unit_type_id: true,
        upgrade_id: true
    }).unwrap();

    let root = Path::new(file!()).parent().unwrap().parent().unwrap();


    {
        let mut units_file = File::create(root.join("src").join("gen").join("units.rs")).unwrap();
        writeln!(units_file, "// generated version controlled file //\nuse super::types::FromU32;\n#[allow(non_camel_case_types)]\n#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash,FromU32)]\npub enum UnitIDs {{").unwrap();

        let mut units = Vec::new();

        for unit in data.units {
            if unit.name.len() > 0 {
                writeln!(units_file, "{} = {},", unit.name, unit.unit_id).unwrap();
            } else {
                writeln!(units_file, "{} = {},", format!("UnitID{}", unit.unit_id), unit.unit_id).unwrap();
            }
            units.push(unit.clone());
        }
        writeln!(units_file, "}}").unwrap();

        let units_json = File::create(root.join("src").join("gen").join("units.json")).unwrap();
        serde_json::to_writer_pretty(units_json, &units).unwrap();
    }
    {
        let mut rs_file = File::create(root.join("src").join("gen").join("abilities.rs")).unwrap();
        writeln!(rs_file, "// generated version controlled file //\nuse super::types::FromU32;\n#[allow(non_camel_case_types)]\n#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash,FromU32)]\npub enum AbilityIDs {{").unwrap();

        let mut datas = Vec::new();

        // naming abilities is a little harder.
        // in possible, use the "Friendly name", but these names are sometimes duplicated, or sometimes missing
        // to prevent duplicates, keep track of all used names and if there is a duplicate, append the ability_id
        let mut names : Vec<String> = Vec::new();

        for abils in data.abilities {
            if let Some(ref friendly_name) = abils.friendly_name {
                if names.contains(friendly_name) {
                    // can't reuse this name, so append the ability id to make it unique

                    if friendly_name.starts_with(|c: char| c.is_numeric()) {
                        writeln!(rs_file, "{}_{} = {},", friendly_name.replace(' ', "_"), abils.ability_id, abils.ability_id).unwrap();
                    } else {
                        writeln!(rs_file, "Abil{}_{} = {},", friendly_name.replace(' ', "_"), abils.ability_id, abils.ability_id).unwrap();
                    }

                } else {
                    if friendly_name.starts_with(|c: char| c.is_numeric()) {
                        writeln!(rs_file, "Abil{} = {},", friendly_name.replace(' ', "_"), abils.ability_id).unwrap();
                    } else {
                        writeln!(rs_file, "{} = {},", friendly_name.replace(' ', "_"), abils.ability_id).unwrap();

                    }
                    names.push(friendly_name.to_owned());

                }
            } else {
                writeln!(rs_file, "AbilityID_{} = {},", abils.ability_id, abils.ability_id).unwrap();

            }

            datas.push(abils.clone());
        }
        writeln!(rs_file, "}}").unwrap();

        let json_file = File::create(root.join("src").join("gen").join("abilities.json")).unwrap();
        serde_json::to_writer_pretty(json_file, &datas).unwrap();
    }
}