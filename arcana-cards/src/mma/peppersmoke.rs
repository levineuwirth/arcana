//! Peppersmoke — `{B}` Kindred Instant — Faerie. "Target creature gets -1/-1
//! until end of turn. If you control a Faerie, draw a card."
//!
//! Note: Type line is "Kindred Instant — Faerie". TypeLine does not have a
//! KINDRED constant shown in the catalog; using INSTANT as best-effort type.
//!
//! GAP: TypeLine::KINDRED not in catalog. The conditional draw ("if you control
//! a Faerie") is also not expressible; Effect::Conditional's condition type
//! is not demonstrated.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Peppersmoke");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        // GAP: TypeLine::KINDRED not in catalog; using INSTANT.
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets -1/-1 until end of turn. If you control a Faerie, draw a card.".into(),
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
    // GAP: Conditional draw based on controlling a Faerie not expressible.
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: -1,
        toughness: -1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
