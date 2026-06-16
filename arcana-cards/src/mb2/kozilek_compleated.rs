//! Kozilek, Compleated — `{8}{C/P}{C/P}` 12/12 Legendary Creature — Phyrexian Eldrazi.
//!
//! COST GAP: the printed cost is `{8}{C/P}{C/P}` (colorless-Phyrexian pips).
//! The mana parser does not accept `{C/P}` (rejects `C` as a slash leaf), so
//! the cost is approximated as `{8}{C}{C}` (same mana value 10, colorless);
//! the "pay with 2 life" Phyrexian rider is dropped.
//! "When you cast this spell, each opponent gets two poison counters, then each
//! opponent with more than two cards in hand discards cards equal to the difference."
//! "Annihinfect (Whenever this creature attacks, defending player sacrifices a
//! permanent for each poison counter they have.)"
//!
//! Both abilities depend on player-directed poison counters, which are not
//! expressible: there is no player-poison Effect (AddCounters is object-only)
//! and no script helper to read a player's poison count. Effects are GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kozilek, Compleated");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}{C}{C}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(12)),
        toughness: Some(PtValue::Fixed(12)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cast_trigger,
                trigger_zones: vec![Zone::Stack],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: annihinfect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn cast_trigger(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "each opponent gets two poison counters, then each opponent with
    // more than two cards in hand discards cards equal to the difference" —
    // no player-directed poison-counter Effect; the per-opponent dynamic
    // discard depends on the poison delta. Whole effect unexpressible.
    Vec::new()
}

fn annihinfect(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "defending player sacrifices a permanent for each poison counter
    // they have" — no script helper to read a player's poison-counter count,
    // so the dynamic sacrifice amount is not computable.
    Vec::new()
}
