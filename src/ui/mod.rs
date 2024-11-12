use std::collections::HashSet;
use dsl::pda;
// use pda::*;
use pda::PushdownAutomaton;

use macroquad::ui::root_ui;

pub fn main_menu() {
    if let sp = root_ui().button(None, "Singleplayer") {
        println!("pressed sp");
    }
    if let mp = root_ui().button(None, "Multiplayer") {
        println!("pressed mp");
    }
}

use rust_fsm::*;

state_machine! {
    #[derive(Debug)]
    user_interface(MainMenu)

    MainMenu(Singleplayer) => Game,
    MainMenu(Multiplayer) => Multiplayer,
    MainMenu(Settings) => Settings,
    MainMenu(Exit) => Exit [Exit],

    Multiplayer(Next) => Game,
    // Multiplayer(Client) => Game,
    // Multiplayer(Server) => Game,
    Multiplayer(Back) => MainMenu,

    Game(Play) => Play,
    Game(Back) => BackState,

    Settings(Back) => MainMenu,
}

// enum MainMenu {
//     Singleplayer,
//     Multiplayer,
//     Options,
//     Exit,
// }

// enum MultiplayerMenu {
//     Join,
//     Host,
// }

// enum GameMenu {
//     New,
//     Load,
// }








pub fn test_ui() {
    pda! {
        Main => Single => Game,
        Main => Multi => Host => Game,
        Main => Multi => Join => Lobby,
        Main => Settings,
    }

    let transitions = generate_transitions();
    println!("{:?}", transitions);

    let start_state = &State::Main;
    let final_states = HashSet::new();
    let mut pda = PushdownAutomaton::new(start_state, final_states, transitions);


    // Test transitions
    println!("0 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::ToSingle), Ok(()));
    println!("1 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::ToGame), Ok(()));
    println!("2 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::Back), Ok(()));
    println!("3 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::Back), Ok(()));
    println!("4 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::ToMulti), Ok(()));
    println!("5 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::ToHost), Ok(()));
    println!("5 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::ToGame), Ok(()));
    println!("7 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::Back), Ok(()));
    println!("8 {:?}", pda.get_state());
    assert_eq!(pda.transition(Input::Back), Ok(()));
    println!("9 {:?}", pda.get_state());
    // assert_eq!(pda.transition(Input::ToSettings), Ok(()));
    // println!("10 {:?}", pda.get_state());
}
