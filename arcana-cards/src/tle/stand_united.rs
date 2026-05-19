//! Stand United — `{1}{G/W}` Instant. "Target creature gets +2/+2 until
//! end of turn. If you control an Ally, scry 2."
//!
//! # Implementation note
//! The +2/+2 pump is expressible. The conditional scry 2 (if you
//! control an Ally) is not expressible as a Conditional without a
//! supported condition variant for subtype-on-battlefield check.
//!
//! # GAP
//! Conditional scry 2 (if you control an Ally) not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stand United");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets +2/+2 until end of turn. If you control an Ally, scry 2.".into(),
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
    vec![
        Effect::Pump {
            target: *id,
            power: 2,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        // GAP: conditional scry 2 (if you control an Ally) not expressible
    ]
}
