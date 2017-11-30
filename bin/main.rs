#![allow(dead_code, unused_imports, unused_variables)]

extern crate sc2;
extern crate sc2_protobuf;
extern crate protobuf;
extern crate ws;
extern crate url;

use url::Url;

use protobuf::Message;

use ws::CloseCode;

use sc2::types;

use std::thread::sleep;
use std::time::Duration;



fn main() {
    // let foo : String = "hello";



    let coord = sc2::Coordinator::new();
    //    let coord = match sc2::Coordinator::connect(Url::parse("ws://localhost:12000/sc2api").unwrap()) {
    //        Ok(c) => c,
    //        Err(e) => {
    //            println!("connection error: {:?}", e);
    //            panic!();
    //        }
    //    };
    let coord = coord.launch().expect("Failed to launch game");
    println!("Game launched, now creating game...");


    // create a 1 player game

    let req = types::RequestCreateGame {
        map: types::RequestMap::LocalMap(types::LocalMap {
            MapPath: "Z:\\devel\\Melee\\Simple64.SC2Map".to_owned(),
        }),
        player_setup: vec![
            types::PlayerSetup {
                field_type: types::PlayerType::Participant,
                race: types::Race::Terran,
                difficulty: types::Difficulty::Easy,
            },
            types::PlayerSetup {
                field_type: types::PlayerType::Computer,
                race: types::Race::Random,
                difficulty: types::Difficulty::Easy,
            },
        ],
        disable_fog: false,
        random_seed: None,
        realtime: true,
    };

    let maps = coord.list_available_maps().unwrap();
    println!("Available maps: {:?}", maps);
    let coord = coord.create_game(req).unwrap();
    println!("Game created!");

    let req = types::RequestJoinGame {
        participation: types::Participation::Race(types::Race::Terran),
        options: types::InterfaceOptions {
            raw: true,
            score: false,
            feature_layer: None,
        },
    };

    let coord = coord.join_game(req).unwrap();
    println!("In game!");


    let data = coord
        .game_data(types::RequestData {
            ability_id: true,
            unit_type_id: true,
            upgrade_id: true,
            buff_id: true,
            effect_id: true,
        })
        .unwrap();
    //println!("Data: {:#?}", data);

    loop {
        sleep(Duration::from_secs(1));
        //coord.step(types::RequestStep { count: 1 });

        let observations : types::ResponseObservation = coord.observation(types::RequestObservation{disable_fog: false}).unwrap();
        let units = observations.observation.raw_data.unwrap();
        println!("{:#?}", units.get_my_units());
        //break;
    }
}
