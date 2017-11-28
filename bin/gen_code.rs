extern crate sc2;
extern crate serde;
extern crate serde_json;
//extern crate quote;


use sc2::types;

use std::fs::File;
use std::io::Write;

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

    let coord = coord.join_game(req).unwrap();


    let data: types::ResponseData = coord.game_data(types::RequestData {
        ability_id: true,
        buff_id: true,
        effect_id: true,
        unit_type_id: true,
        upgrade_id: true
    }).unwrap();


    {
        let mut units_file = File::create("src/gen/units.rs").unwrap();
        writeln!(units_file, "// generated version controlled file //\nuse super::types::FromU32;\n#[allow(non_camel_case_types)]\n#[derive(Debug,Eq,PartialEq,Hash,FromU32)]\npub enum UnitIDs {{");

        let mut units = Vec::new();

        for unit in data.units {
            if unit.name.len() > 0 {
                writeln!(units_file, "{} = {},", unit.name, unit.unit_id);
            } else {
                writeln!(units_file, "{} = {},", format!("UnitID{}", unit.unit_id), unit.unit_id);
            }
            units.push(unit.clone());
        }
        writeln!(units_file, "}}");

        let mut units_json = File::create("src/gen/units.json").unwrap();
        serde_json::to_writer_pretty(units_json, &units).unwrap();
    }
    {
        let mut rs_file = File::create("src/gen/abilities.rs").unwrap();
        writeln!(rs_file, "// generated version controlled file //\nuse super::types::FromU32;\n#[allow(non_camel_case_types)]\n#[derive(Debug,Eq,PartialEq,Hash,FromU32)]\npub enum AbilityIDs {{");

        let mut datas = Vec::new();

        for abils in data.abilities {
            if abils.link_name.len() > 0 {
                if abils.link_name.chars().next().unwrap().is_numeric() {
                    writeln!(rs_file, "{} = {},", format!("A{}{}", abils.link_name.replace(' ', "_"), abils.ability_id), abils.ability_id);
                } else {
                    writeln!(rs_file, "{} = {},", format!("{}{}", abils.link_name.replace(' ', "_"), abils.ability_id), abils.ability_id);
                }
            } else {
                writeln!(rs_file, "{} = {},", format!("AbilityID{}", abils.ability_id), abils.ability_id);
            }
            datas.push(abils.clone());
        }
        writeln!(rs_file, "}}");

        let mut json_file = File::create("src/gen/abilities.json").unwrap();
        serde_json::to_writer_pretty(json_file, &datas).unwrap();
    }
}