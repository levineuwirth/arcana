//! The Nipton Lottery — `{2}{B}{R}` sorcery. "Choose a creature at
//! random. You gain control of that creature until end of turn.
//! Untap it. It gains haste until end of turn. Then destroy all other
//! creatures." 'Choose at random' isn't a script primitive; even if
//! we had it, ChangeControl is permanent — no Threaten-style EOT
//! variant. Best-effort: destroy all creatures (the back half); GAP
//! the random pick + temporary control + untap + haste.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Nipton Lottery");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    // GAP: 'choose at random' + temporary control + untap + haste on the picked creature.
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose a creature at random. You gain control of that creature until end of turn. Untap it. It gains haste until end of turn. Then destroy all other creatures.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ForEach {
        targets: script::ids_matching(state, &ObjectFilter::creature(), entry.controller),
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
