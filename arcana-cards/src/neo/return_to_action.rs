//! Return to Action — `{1}{B}` Instant. "Until end of turn, target
//! creature gets +1/+0 and gains lifelink and \"When this creature
//! dies, return it to the battlefield tapped under its owner's
//! control.\""
//!
//! # Implementation note
//! Pump with +1/+0 and lifelink is expressible. The dies trigger
//! granting tapped return-to-battlefield is not expressible (no API
//! to attach a temporary triggered ability to an object).
//!
//! # GAP
//! Granting a temporary "when dies, return tapped" triggered ability
//! not in Effect catalog.

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
    let name = reg.interner_mut().intern("Return to Action");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Until end of turn, target creature gets +1/+0 and gains lifelink and \"When this creature dies, return it to the battlefield tapped under its owner's control.\"".into(),
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
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Lifelink],
        },
        // GAP: granting temporary "when dies, return tapped" triggered ability not in catalog
    ]
}
