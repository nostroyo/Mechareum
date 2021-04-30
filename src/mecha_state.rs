use crate::mecha;

pub struct MechaState {
    pub name: String,
    pub DoApplyState: fn(&mut mecha::Mecha),
    pub DoAfterApply: Option<fn(&mut mecha::Mecha)>,
    nb_turn_apply: u8,
}

impl MechaState {
    fn new (state_name: &str, apply_state: fn(&mut mecha::Mecha), DoAfterApply: Option<fn(&mut mecha::Mecha)>, nb_turn_apply: u8) -> MechaState {
        MechaState {
            name: state_name.to_string(),
            DoApplyState: apply_state,
            DoAfterApply,
            nb_turn_apply
        }
    }
}

fn get_all_available_state() -> [MechaState; 2] {
    return [
        MechaState::new("Virus", |mecha| mecha.TakeDamage(1), None, 255),
        MechaState::new("Stunned", |mecha| mecha.current_atk = 0, Some(|mecha| mecha.current_atk = mecha.atk), 1),
    ];
}
