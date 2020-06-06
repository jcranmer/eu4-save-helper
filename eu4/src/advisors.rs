use paradox::{ParadoxParse};
use crate::{CountryModifier, Weight};

#[derive(ParadoxParse, Default)]
pub struct AdvisorType {
    pub monarch_power: (),
    pub skill_scaled_modifier: Vec<CountryModifier>,
    pub chance: (), // XXX: this is a province Weight,
    pub ai_will_do: Weight,

    #[modifiers] pub bonus: Vec<CountryModifier>
}
