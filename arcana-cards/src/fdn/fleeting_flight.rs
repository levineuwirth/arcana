//! Fleeting Flight — `{W}` instant. "Put a +1/+1 counter on target creature.
//! It gains flying until end of turn. Prevent all combat damage that would be
//! dealt to it this turn."
//!
//! # GAP: prevent-combat-damage effect is not in the engine catalog.
//! The +1/+1 counter and flying grant are implemented; the damage-prevention
//! clause returns Vec::new() in isolation but is folded into the same resolver.
//! We emit AddCounters + GrantKeyword and note the gap.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fleeting Flight");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter on target creature. It gains flying until end of turn. Prevent all combat damage that would be dealt to it this turn.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: prevent-combat-damage effect is not expressible via the catalog
    vec![
        Effect::AddCounters { target: *id, kind: CounterKind::PlusOnePlusOne, count: 1 },
        Effect::GrantKeyword { target: *id, keyword: KeywordAbility::Flying, duration: Duration::EndOfTurn },
    ]
}
