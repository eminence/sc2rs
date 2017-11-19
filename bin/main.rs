#![allow(dead_code, unused_imports, unused_variables)]

extern crate sc2;
extern crate sc2_protobuf;
extern crate protobuf;

use protobuf::Message;

extern crate ws;

use ws::CloseCode;

use std::thread::sleep;
use std::time::Duration;

/*

Sending: create_game {
  local_map {
    map_path: "Z:\\devel\\s2client-api\\maps\\Ladder/(2)Bel\'ShirVestigeLE (Void).SC2Map"
  }
  player_setup {
    type: Participant
    race: Terran
    difficulty: Easy
  }
  player_setup {
    type: Computer
    race: Zerg
    difficulty: Easy
  }
  realtime: false
}

*/

fn main() {

   // let foo : String = "hello";

    let coord = sc2::Coordinator::new();
    let coord = coord.launch().expect("Failed to launch game");

    //let coord = coord.create_game();

//    sc2::launch();
//    sleep(Duration::from_secs(10));
//
//    ws::connect("ws://127.0.0.1:8167/sc2api", |out| {
//        let mut local_map = sc2_protobuf::protos::LocalMap::new();
//        local_map.set_map_path("Z:\\devel\\s2client-api\\maps\\Ladder/(2)Bel\'ShirVestigeLE (Void).SC2Map".to_owned());
//
//
//        let mut player_setup_list = protobuf::repeated::RepeatedField::new();
//
//
//        let mut player_setup = sc2_protobuf::protos::PlayerSetup::new();
//        player_setup.set_field_type(sc2_protobuf::protos::PlayerType::Participant);
//        player_setup.set_race(sc2_protobuf::protos::Race::Terran);
//        player_setup.set_difficulty(sc2_protobuf::protos::Difficulty::Easy);
//        player_setup_list.push(player_setup);
//
//        let mut player_setup = sc2_protobuf::protos::PlayerSetup::new();
//        player_setup.set_field_type(sc2_protobuf::protos::PlayerType::Computer);
//        player_setup.set_race(sc2_protobuf::protos::Race::Zerg);
//        player_setup.set_difficulty(sc2_protobuf::protos::Difficulty::Easy);
//        player_setup_list.push(player_setup);
//
//        let mut rcg = sc2_protobuf::protos::RequestCreateGame::new();
//        rcg.set_local_map(local_map);
//        rcg.set_player_setup(player_setup_list);
//        rcg.set_realtime(false);
//
//
//        let mut req = sc2_protobuf::protos::Request::new();
//        req.set_create_game(rcg);
//
//        let mut v = Vec::new();
//        req.write_to_vec(&mut v);
//
//        out.send(ws::Message::binary(v)).unwrap();
//
//        move |msg| {
//            println!("Got msg: {}", msg);
//            if let ws::Message::Binary(bin) = msg {
//                let mut resp = sc2_protobuf::protos::Response::new();
//                resp.merge_from_bytes(&bin).expect("Merge failed");
//                println!("Resp: {:?}", resp);
//
//                /*
//                Sending: join_game {
//  race: Terran
//  options {
//    raw: true
//    score: true
//  }
//}
//*/
//                let mut opts = sc2_protobuf::protos::InterfaceOptions::new();
//                opts.set_raw(true);
//                opts.set_score(true);
//
//                let mut join = sc2_protobuf::protos::RequestJoinGame::new();
//                join.set_race(sc2_protobuf::protos::Race::Terran);
//                join.set_options(opts);
//
//                let mut req = sc2_protobuf::protos::Request::new();
//                req.set_join_game(join);
//
//                let mut v = Vec::new();
//                req.write_to_vec(&mut v);
//
//                out.send(ws::Message::binary(v))
//            } else {
//                out.close(CloseCode::Normal)
//
//            }
//
//
//
//        }
//    }).unwrap();

    sleep(Duration::from_secs(30));
}