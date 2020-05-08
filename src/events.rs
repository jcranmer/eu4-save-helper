use crate::lexer::{NullScope, ParadoxScope, Token};

pub struct EventList {
    events: Vec<Event>,
    requires_normal: bool
}

impl Default for EventList {
    fn default() -> EventList {
        EventList { events: Default::default(), requires_normal: false }
    }
}

impl ParadoxScope for EventList {
    fn start_scope(&mut self, id: Token) -> &mut ParadoxScope {
        let scope = if id.is("country_event") {
            Scope::Country
        } else if id.is("province_event") {
            Scope::Province
        } else {
            return NullScope::instance();
        };

        self.events.push(Event::new(scope));
        let last_index = self.events.len() - 1;
        &mut self.events[last_index]
    }

    fn set_property(&mut self, id: Option<Token>, value: Token) {
        if let Some(id) = id {
            if id.is("normal_or_historical_nations") {
                self.requires_normal = value.is("yes");
            } else if id.is("namespace") {
                // Do nothing
            } else {
                // Print error message saying unknown key.
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Scope {
    Country,
    Province
}

pub struct Event {
    scope: Scope,
    id: String,
    title: String, // Localized
    triggers: Vec<Condition>,
    hidden: bool,
    major: bool,
    unique: bool,
    forced: bool,
}

impl Event {
    pub fn new(scope: Scope) -> Event {
        Event {
            scope,
            id: Default::default(),
            title: Default::default(),
            triggers: Default::default(),
            hidden: false,
            major: false,
            unique: false,
            forced: false,
        }
    }
}

impl ParadoxScope for Event {
    fn start_scope(&mut self, id: Token) -> &mut ParadoxScope {
        if id.is("trigger") {
            &mut self.triggers
        } else {
            println!("Event/{:?}", id);
            NullScope::instance()
        }
    }

    fn set_property(&mut self, id: Option<Token>, value: Token) {
        if let Some(id) = id {
            if id.is("id") {
                self.id = value.into();
            } else if id.is("title") {
                self.title = value.into();
            } else if id.is("hidden") {
                self.hidden = value.into();
            } else if id.is("major") {
                self.major = value.into();
            } else if id.is("fire_only_once") {
                self.unique = value.into();
            } else if id.is("is_triggered_only") {
                self.forced = value.into();
            } else {
                println!("Event/{:?}", Some(id));
            }
        } else {
            println!("Event/{:?}", id);
        }
    }
}

enum Condition {
    All(Vec<Condition>),
    Any(Vec<Condition>),
    Not(Vec<Condition>),
    Scoped()
}

impl ParadoxScope for Vec<Condition> {
    fn start_scope(&mut self, id: Token) -> &mut ParadoxScope {
        let new_condition = if id.is("AND") {
            Condition::All(Vec::new())
        } else if id.is("OR") {
            Condition::Any(Vec::new())
        } else if id.is("NOT") {
            Condition::Not(Vec::new())
        } else {
            println!("Condition/{:?}", id);
            return NullScope::instance();
        };
        self.push(new_condition);
        match self.last_mut().unwrap() {
            Condition::All(ref mut t) => t,
            Condition::Any(ref mut t) => t,
            Condition::Not(ref mut t) => t,
            _ => panic!("Shouldn't reach here")
        }
    }

    fn set_property(&mut self, id: Option<Token>, value: Token) {
        if let Some(id) = id {
                println!("Condition/{:?}", Some(id));
        } else {
            println!("Condition/{:?}", id);
        }
    }
}
