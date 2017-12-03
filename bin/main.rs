#![allow(dead_code, unused_imports, unused_variables)]

extern crate sc2;
extern crate sc2_protobuf;
extern crate protobuf;
extern crate url;

use url::Url;

use protobuf::Message;


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
    let mut coord = coord.launch().expect("Failed to launch game");
    println!("Game launched, now creating game...");

    coord.list_available_maps();


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

    let mut coord = coord.join_game(req).unwrap();
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


    let mut count = 0usize;
    loop {
        count += 1;
        sleep(Duration::from_millis(1000));
        //coord.step(types::RequestStep { count: 2 });
        //if count % 50 == 0 {
            println!("Getting an observation...");
            let start = std::time::Instant::now();
            let observations: types::ResponseObservation = coord.observation(types::RequestObservation { disable_fog: false }).unwrap();
            let end = std::time::Instant::now();
            let dur = end - start;
            println!("got observation for game_loop {}, this took {:?} ", observations.observation.game_loop, dur.as_secs());
        //}

        //let units = observations.observation.raw_data.unwrap();
        //let my_units = units.get_my_units();
        //let selected_units : Vec<_> = my_units.iter().filter(|u| u.is_selected).collect();
        // println!("{:#?}", selected_units);
        //break;
    }
}
