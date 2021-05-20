use std::mem::transmute;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum MechaColor {
    Red = 0,
    Blue,
    Green,
    Yellow,
    Last,
}

pub type BackendMechaCharacteristics = (u8, u8, u8, u8, String);

#[derive(Clone)]
pub struct Mecha{
    pub name: String,

    // blockchain generated attribute
    pub atk: u8,
    pub health: u8,
    pub speed: u8,
    color: MechaColor,

    xp: u8,
    pub current_atk: u8,

}
impl Mecha {
   pub fn new(name: String, atk: u8, health: u8, speed: u8, color: MechaColor) -> Mecha {
        Mecha {
            name,
            health,
            atk,
            speed,
            xp: 0,
            current_atk: atk,
            color
        }
    }

    pub fn NewFromBlockchain(characteristics: (u8, u8, u8, u8, String)) -> Option<Mecha> {
        let mecha_color: MechaColor = unsafe {transmute(characteristics.3)};

        Some(Mecha::new(
            characteristics.4,
            characteristics.0,
            characteristics.1,
            characteristics.2,
            mecha_color))
    }

    pub fn TakeDamage(&mut self, amount_damage: u8) {
        if amount_damage > self.health {
            self.health = 0
        } else {
            self.health -= amount_damage;
        }
    }

    pub fn AttackOpponent(&self, damaging_mecha: &mut Mecha) {
        let mut atk = self.current_atk;
        if atk > 0 {
            match damaging_mecha.color {
                MechaColor::Red => {
                    match self.color {
                        MechaColor::Red => {}
                        MechaColor::Blue => { atk += 1 }
                        MechaColor::Green => {}
                        MechaColor::Yellow => { atk -= 1 }
                        _ => {panic!("Invalid Mecha color")}
                    }
                }
                MechaColor::Blue => {
                    match self.color {
                        MechaColor::Red => { atk -= 1 }
                        MechaColor::Blue => {}
                        MechaColor::Green => {}
                        MechaColor::Yellow => { atk += 1 }
                        _ => {panic!("Invalid Mecha color")}
                    }
                }
                MechaColor::Green => {}
                MechaColor::Yellow => {
                    match self.color {
                        MechaColor::Red => { atk += 1 }
                        MechaColor::Blue => { atk -= 1 }
                        MechaColor::Green => {}
                        MechaColor::Yellow => {}
                        _ => {panic!("Invalid Mecha color")}
                    }
                }
                _ => {panic!("Invalid Mecha color")}
            }
            damaging_mecha.TakeDamage(atk);
        }
    }

    pub fn info(& self) -> String {
        format!("Name: {} Health: {} current atk: {} ", self.name, self.health, self.current_atk)
    }
}


#[cfg(test)]
mod tests {
    use crate::mecha::{Mecha, MechaColor};

    #[test]
    fn test_damage_exceed_life(){
        let mut mecha_test = Mecha::new(String::from("mecha test"), 1, 1, 1, MechaColor::Yellow);
        mecha_test.TakeDamage(2);

        assert_eq!(mecha_test.health, 0);
    }

    #[test]
    fn test_damage(){
        let mut mecha_test = Mecha::new(String::from("mecha test"), 1, 1, 1, MechaColor::Yellow);
        mecha_test.TakeDamage(1);

        assert_eq!(mecha_test.health, 0);

        let mut mecha_test = Mecha::new(String::from("mecha test"), 2, 1, 1, MechaColor::Yellow);
        mecha_test.TakeDamage(1);

        assert_eq!(mecha_test.health, 1);
    }

    #[test]
    fn test_damage_opponent_weakness() {
        let attacking_mecha = Mecha::new(String::from("mecha test"), 1, 2, 1, MechaColor::Yellow);
        let defending_mecha = &mut Mecha::new(String::from("mecha test"), 1, 1, 1, MechaColor::Red);

        attacking_mecha.AttackOpponent(defending_mecha);

        assert_eq!(defending_mecha.health, 0);
    }

    #[test]
    fn test_damage_opponent_strength() {
        let attacking_mecha = Mecha::new(String::from("mecha test"), 1, 2, 1, MechaColor::Yellow);
        let defending_mecha = &mut Mecha::new(String::from("mecha test"), 3, 1, 1, MechaColor::Blue);

        attacking_mecha.AttackOpponent(defending_mecha);

        assert_eq!(defending_mecha.health, 0);
    }
}