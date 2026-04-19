//! Abrade — `{1}{R}` modal instant. "Choose one — Abrade deals 3
//! damage to target creature; or destroy target artifact." The
//! canonical seed proof of the CR 700.2 modal pipeline:
//! per-clause target filters, one-clause-is-chosen semantics, and
//! effect dispatch branching on the chosen mode.
//!
//! # Rules references
//!
//! * CR 700.2 — A modal spell's caster picks `[min_modes, max_modes]`
//!   clauses at cast time. Abrade is "Choose one" (min=1, max=1).
//! * CR 700.2c — Modes resolve in the order printed on the card,
//!   not the order the caster picked. Engine-side, this is the
//!   sorted-ascending invariant on [`arcana_core::stack::ModeChoice`].
//! * CR 608.2b — On resolution, each chosen target is rechecked
//!   against the clause's target requirement. The engine's modal-
//!   aware `effective_target_requirements` concatenates the chosen
//!   clauses' requirements in card order, so the recheck composes
//!   without Abrade needing to do anything special.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardRegistry, ModalSpec, ModeClause, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Abrade");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose one — Abrade deals 3 damage to target \
                       creature; or destroy target artifact.".into(),
                // Non-modal flat targets is empty for modal spells;
                // the effective targets come from the chosen clauses.
                target_requirements: vec![],
                modal: Some(ModalSpec {
                    min_modes: 1,
                    max_modes: 1,
                    clauses: vec![
                        ModeClause {
                            text: "Abrade deals 3 damage to target creature."
                                .into(),
                            target_requirements: vec![
                                TargetRequirement::target_creature(),
                            ],
                        },
                        ModeClause {
                            text: "Destroy target artifact.".into(),
                            target_requirements: vec![TargetRequirement {
                                filter: TargetFilter::Permanent(ObjectFilter {
                                    types: Some(TypeLine::ARTIFACT.into()),
                                    ..Default::default()
                                }),
                                count: TargetCount::Exactly(1),
                                controller: None,
                            }],
                        },
                    ],
                }),
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Modal dispatch: one mode chosen (min=max=1). `mode_indices` is
    // kept sorted ascending by `ModeChoice::new`, so reading
    // `first()` is equivalent to "the chosen index."
    let Some(choice) = entry.modes.first() else { return Vec::new(); };
    let Some(&mode_idx) = choice.mode_indices.first() else { return Vec::new(); };
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let target_obj = match target {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };
    match mode_idx {
        0 => vec![Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(target_obj),
            amount: 3,
        }],
        1 => vec![Effect::DestroyPermanent { target: target_obj }],
        _ => Vec::new(),
    }
}
