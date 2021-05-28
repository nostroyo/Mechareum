use crate::mecha::{Mecha, MechaColor, BackendMechaCharacteristics};
use crate::backend_mecha_function::BackEndMechaFunction;
use web3::types::U256;
use rand::{thread_rng, Rng, ThreadRng};
use std::mem::transmute;


const STANDARD_MIN: u8 = 1;
const STANDARD_MAX: u8 = 3;
const BOOSTED_MIN: u8 = 3;
const BOOSTED_MAX: u8 = 5;

pub struct MockBackend {
    mecha_list: Vec<BackendMechaCharacteristics>,
    randomizer: ThreadRng,
}

impl BackEndMechaFunction for MockBackend {
    fn get_mecha_characteristics_by_id(&self, id: U256) -> BackendMechaCharacteristics {
        self.mecha_list.get(id.as_usize()).unwrap().clone()
    }

    fn generate_new_mecha(&mut self, mecha_name: String) {

        let mut color: MechaColor = unsafe {transmute(self.randomizer.gen_range(0, MechaColor::Last as u8))};
        let mut atk_health_speed: (u8, u8, u8) = Default::default();
        match color {
            MechaColor::Red => {
                atk_health_speed.0 = self.randomizer.gen_range(BOOSTED_MIN, BOOSTED_MAX);
                atk_health_speed.1 = self.randomizer.gen_range(STANDARD_MIN, STANDARD_MAX);
                atk_health_speed.2 = self.randomizer.gen_range(STANDARD_MIN, STANDARD_MAX);
            }
            MechaColor::Blue => {
                atk_health_speed.0 = self.randomizer.gen_range(STANDARD_MIN, STANDARD_MAX);
                atk_health_speed.1 = self.randomizer.gen_range(BOOSTED_MIN, BOOSTED_MAX);
                atk_health_speed.2 = self.randomizer.gen_range(STANDARD_MIN, STANDARD_MAX);
            }
            MechaColor::Green => {
                atk_health_speed.0 = self.randomizer.gen_range(STANDARD_MIN, STANDARD_MAX);
                atk_health_speed.1 = self.randomizer.gen_range(STANDARD_MIN, STANDARD_MAX);
                atk_health_speed.2 = self.randomizer.gen_range(STANDARD_MIN, STANDARD_MAX);
            }
            MechaColor::Yellow => {
                atk_health_speed.0 = self.randomizer.gen_range(STANDARD_MIN, STANDARD_MAX);
                atk_health_speed.1 = self.randomizer.gen_range(STANDARD_MIN, STANDARD_MAX);
                atk_health_speed.2 = self.randomizer.gen_range(BOOSTED_MIN, BOOSTED_MAX);
            }
            _ => {panic!("Invalid mecha color")}
        }

        self.mecha_list.push((
            atk_health_speed.0,
            atk_health_speed.1,
            atk_health_speed.2,
            color as u8,
            format!("{}", self.mecha_list.len()))
        );

    }

    fn get_total_mecha_owned(&mut self) -> U256 {
        U256::from(self.mecha_list.len())
    }

    fn get_owned_mecha_by_index(&mut self, id: U256) -> BackendMechaCharacteristics {
        self.get_mecha_characteristics_by_id(id)
    }
}

impl MockBackend {
    pub fn new() -> Self {
        MockBackend {
            mecha_list: vec![],
            randomizer: rand::thread_rng()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mock_backend_function::MockBackend;
    use crate::backend_mecha_function::BackEndMechaFunction;
    use web3::types::U256;

    #[test]
    fn test_add_mecha() {

        let mut backend = MockBackend::new();

        for i in 0..5 {
            backend.generate_new_mecha(format!("{}", i))
        }

        {
            let mecha_list = & mut backend.mecha_list;
            for mecha in mecha_list {
                println!("{}", mecha.4)
            }
        }

        assert_eq!(backend.get_total_mecha_owned(), U256::from(5));

        let mecha = backend.get_owned_mecha_by_index(U256::from(0));
        assert_eq!(mecha.4, String::from("0"))

    }

}