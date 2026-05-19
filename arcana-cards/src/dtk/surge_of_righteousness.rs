//! Surge of Righteousness — `{1}{W}` instant. "Destroy target black or
//! red creature that's attacking or blocking. You gain 2 life."
//!
//! # GAP
//! "Attacking or blocking" is not a supported TargetFilter predicate.
//! Color filter (black or red) via `ObjectFilter::with_colors` uses
//! exact-match semantics; "black or red" as an OR requires
//! `with_types_any` analog for colors, which is not shown. Best
//! effort: plain creature target; the gain-2-life rider is fully
//! modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Surge of Righteousness");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target black or red creature that's attacking or blocking. You gain 2 life.".into(),
                // GAP: attacking/blocking filter and black-or-red color filter not supported; using plain creature target
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
        Effect::DestroyPermanent { target: *id },
        Effect::GainLife { player: entry.controller, amount: 2 },
    ]
}
