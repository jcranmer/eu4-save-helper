use crate::{Eu4Atom, Modifiers, Weight};
use paradox::ParadoxParse;

#[derive(ParadoxParse, Default)]
pub struct IdeaGroup {
    #[optional] pub start: Modifiers,
    pub bonus: Modifiers,
    #[optional] pub trigger: (),
    #[optional] pub free: bool,
    #[optional] pub ai_will_do: Weight,
    #[optional] pub important: bool,
    #[optional] pub category: Eu4Atom,

    #[collect] pub ideas: Vec<(Eu4Atom, Modifiers)>
}

impl IdeaGroup {
    pub fn add_idea_modifiers(&self, count: i32, modifiers: &mut Modifiers) {
        modifiers.add_modifiers(&self.start);
        let count = std::cmp::min(count as usize, self.ideas.len());
        for (_, value) in &self.ideas[0..count] {
            modifiers.add_modifiers(value);
        }
        if count == self.ideas.len() {
            modifiers.add_modifiers(&self.bonus);
        }
    }
}

#[derive(ParadoxParse, Default)]
pub struct Policy {
    monarch_power: Eu4Atom,
    potential: (), // Condition
    allow: (), // Condition
    ai_will_do: Weight,

    #[modifiers] pub modifiers: Modifiers,
}

#[derive(ParadoxParse, Default)]
pub struct GovernmentReform {
    #[optional] icon: String,
    #[optional] pub modifiers: Modifiers,
    #[optional] ai: (),
    #[optional] potential: (),
    #[optional] conditional: (),
    #[optional] trigger: (),

    #[optional] allow_normal_conversion: bool,
    #[optional] allow_convert: bool,
    #[optional] valid_for_new_country: bool,
    #[optional] effect: (),
    #[optional] removed_effect: (),
    #[optional] post_removed_effect: (),
    #[optional] lock_level_when_selected: bool,
    #[optional] legacy_equivalent: Eu4Atom,
    #[optional] replacement_on_independence_war: Eu4Atom,

    #[optional] valid_for_nation_designer: bool,
    #[optional] nation_designer_cost: i32,
    #[optional] nation_designer_trigger: (),
    #[optional] custom_attributes: (),

    #[optional] assimilation_cultures: (),
    #[optional] factions: (),
    #[optional] government_abilities: (),
    #[optional] states_general_mechanic: (),
    #[optional] disallowed_trade_goods: (),
    #[optional] trade_city_reform: String,
    #[modifiers] effect_modifiers: Modifiers,
}
