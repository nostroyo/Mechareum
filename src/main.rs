use crate::mecha::{Mecha, MechaColor};


mod mecha;
mod mecha_state;
mod ethBridge;

fn main() {
    let mut nbTurn = 1;

    let mut battling_mecha = [
        Mecha::new(String::from("YTH"), 25, 1, 1, MechaColor::Red),
        Mecha::new(String::from("CPU"), 2, 2, 2, MechaColor::Blue),
    ];


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

