//! Invade the City — `{1}{U}{R}` sorcery. "Amass Zombies X, where X is the
//! number of instant and sorcery cards in your graveyard."
//!
//! Implemented with [`Effect::Amass`]: grow (or create) a black Zombie Army and
//! put X +1/+1 counters on it, where X is computed at resolution as the number
//! of instant and sorcery cards in the controller's graveyard via
//! [`script::graveyard_matching`].

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invade the City");
    let _army = reg.interner_mut().intern("Army");
    let _zombie = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Amass Zombies X, where X is the number of instant and sorcery \
                   cards in your graveyard."
                .into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let army_sub = reg
        .interner()
        .lookup("Army")
        .expect("Army interned during register()");
    let zombie_sub = reg
        .interner()
        .lookup("Zombie")
        .expect("Zombie interned during register()");

    let x = script::graveyard_matching(
        state,
        &ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
        entry.controller,
        entry.controller,
    );

    vec![Effect::Amass {
        controller: entry.controller,
        count: x,
        army_subtype: army_sub,
        race_subtype: zombie_sub,
    }]
}
