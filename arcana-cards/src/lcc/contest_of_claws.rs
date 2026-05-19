//! Contest of Claws — `{1}{G}` sorcery. "Target creature you control deals
//! damage equal to its power to another target creature. If excess damage was
//! dealt this way, discover X, where X is that excess damage."
//!
//! # GAP: Fight variant using a creature's power value — Fight uses creature's
//! actual power, which is correct, but there is no way to get the pre-fight
//! toughness to compute excess damage.
//! # GAP: Discover X where X is excess damage — no Effect::Discover variant.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Contest of Claws");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to another target creature. If excess damage was dealt this way, discover X, where X is that excess damage.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_creature(),
                ],
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
    let targets = &entry.targets.targets;
    if targets.len() < 2 { return Vec::new(); }
    let (Some(a_id), Some(b_id)) = (
        if let TargetChoice::Object(id) = &targets[0] { Some(*id) } else { None },
        if let TargetChoice::Object(id) = &targets[1] { Some(*id) } else { None },
    ) else { return Vec::new(); };
    vec![
        Effect::Fight { a: a_id, b: b_id },
        // GAP: Discover X where X is excess damage from the fight — no Effect::Discover
    ]
}
