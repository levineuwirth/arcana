//! Basri's Aegis — `{2}{W}{W}` sorcery. "Put a +1/+1 counter on each of up to
//! two target creatures. You may search your library and/or graveyard for a card
//! named Basri, Devoted Paladin, reveal it, and put it into your hand. If you
//! search your library this way, shuffle."
//!
//! GAP: "search library and/or graveyard for a card named X" — TutorToHand only
//! supports ObjectFilter (type-based), not name-based search. Also "and/or graveyard"
//! search combo is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Basri's Aegis");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter on each of up to two target creatures. You may search your library and/or graveyard for a card named Basri, Devoted Paladin, reveal it, and put it into your hand. If you search your library this way, shuffle.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: name-based library/graveyard search not expressible
    let mut effects = Vec::new();
    for target in &entry.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            });
        }
    }
    effects
}
