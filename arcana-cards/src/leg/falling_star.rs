//! Falling Star — `{2}{R}` sorcery. The dexterity card: physically flip
//! the card; it deals 3 damage to each creature it lands on and taps all
//! creatures dealt damage, with no effect if it doesn't turn over.
//!
//! The physical-flip / "lands on" / "turns completely over" mechanic is
//! not modelable in a deterministic digital engine. We approximate the
//! game-state-affecting part: deal 3 damage to each creature and tap each
//! creature (the creatures dealt damage). The dexterity gate (height,
//! turn-over, which creatures it lands on) is a GAP.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Falling Star");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Falling Star deals 3 damage to each creature it lands on. \
                       Tap all creatures dealt damage by Falling Star."
                    .into(),
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
    // GAP: the physical-flip dexterity mechanic ("lands on", "turns
    // completely over", height) cannot be modeled; we approximate by
    // affecting every creature.
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![
        Effect::ForEach {
            targets: ids.clone(),
            effect: Box::new(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(NULL_OBJECT_ID),
                amount: 3,
            }),
        },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::Tap { target: NULL_OBJECT_ID }),
        },
    ]
}
