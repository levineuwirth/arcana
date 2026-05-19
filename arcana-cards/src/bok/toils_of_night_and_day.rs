//! Toils of Night and Day — `{2}{U}` instant — Arcane.
//! "You may tap or untap target permanent, then you may tap or untap another target permanent."
//!
//! The type line is "Instant — Arcane"; the engine does not have an Arcane subtype constant so
//! we record the subtype string via intern but note the Arcane subtype is not separately modeled.
//! The "may tap or untap" is expressed as Tap for both targets (best effort; the optional choice
//! is dropped — we always tap both targets as a best effort stub, noting the full logic requires
//! conditional Effects not available).
//!
//! # GAP: OptionalTapOrUntapChoice — no Effect variant for player-optional tap-or-untap choice
//! on a target. Best effort: tap both targets.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Toils of Night and Day");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You may tap or untap target permanent, then you may tap or untap another target permanent.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::default()),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::default()),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
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
    // GAP: OptionalTapOrUntapChoice — no Effect variant for "you may tap or untap" player choice;
    // emitting Tap for each target as best effort.
    entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::Tap { target: *id })
        } else {
            None
        }
    }).collect()
}
