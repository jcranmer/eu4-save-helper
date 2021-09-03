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
