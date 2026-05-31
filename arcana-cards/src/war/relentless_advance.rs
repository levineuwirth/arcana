//! Relentless Advance — `{3}{U}` sorcery. "Amass Zombies 3." (Put three
//! +1/+1 counters on an Army you control. It's also a Zombie. If you
//! don't control an Army, create a 0/0 black Zombie Army creature token
//! first.)
//!
//! Implemented with the fused `Effect::Amass`, which grows an existing
//! Army (lowest object id) by `count` +1/+1 counters or mints a fresh
//! 0/0 black Army token of the given race subtype and loads it with the
//! counters. The "Army" and "Zombie" subtype handles are interned in
//! `register` and looked up in the resolver.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Relentless Advance");
    let _army = reg.interner_mut().intern("Army");
    let _zombie = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Amass Zombies 3.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let army = reg
        .interner()
        .lookup("Army")
        .expect("Army interned during register()");
    let zombie = reg
        .interner()
        .lookup("Zombie")
        .expect("Zombie interned during register()");
    vec![Effect::Amass {
        controller: entry.controller,
        count: 3,
        army_subtype: army,
        race_subtype: zombie,
    }]
}
