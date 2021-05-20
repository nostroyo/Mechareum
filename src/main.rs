use crate::mecha::{Mecha, MechaColor};
use crate::mecha_collection::CachedMechaCollection;

mod mecha;
mod mecha_state;
mod ethBridge;
mod mecha_collection;
mod backend_mecha_function;
mod mock_backend_function;


fn main() {
    let mut nbTurn = 1;

    let mut battling_mecha = [
        Mecha::new(String::from("YTH"), 25, 1, 1, MechaColor::Red),
        Mecha::new(String::from("CPU"), 2, 2, 2, MechaColor::Blue),
    ];

    //let mut user_mecha_list: Option<CachedMechaCollection> = CachedMechaCollection::new ;



    loop {
        print!("Turn {} ", nbTurn);

        if let Some((First, Elements)) = battling_mecha.split_first_mut() {

            if let Some(Second) = Elements.get_mut(0) {

                let fastest;
                let slowest;
                if First.speed >= Second.speed {
                    fastest = First;
                    slowest = Second;
                } else {
                    fastest = Second;
                    slowest = First;
                }

                println!("{} attack", fastest.name);
                fastest.AttackOpponent(slowest);
                if CheckKO(slowest){
                    break;
                }

                println!("{} attack", slowest.name);
                slowest.AttackOpponent(fastest);
                if CheckKO(fastest){
                    break;
                }
            }
        }
        print_info(&battling_mecha);

        nbTurn += 1;
    }
}

fn CheckKO(mecha: &Mecha) -> bool {
    if mecha.health == 0 {
        println!("{} KO", mecha.name);
        true
    } else {
        false
    }
}

fn print_info(battling_mecha: &[Mecha]) {
    for mecha in battling_mecha {
        println!("stats: {}", mecha.info());
        println!("stats: {}", mecha.info());
    }

}

