//! Esper Sentinel — `{W}` 1/1 white Artifact Creature — Human Soldier.
//! "Whenever an opponent casts their first noncreature spell each turn,
//! draw a card unless that player pays {X}, where X is Esper Sentinel's
//! power."
//!
//! # GAP: The mana cost in `OptionalPaymentKind::Mana` must be a static
//! `ManaCost` value. The payment "{X} where X is this creature's power"
//! is dynamic (computed at resolution from the permanent's live power).
//! The engine does not support dynamic-mana OptionalPayment costs.
//! Implemented as: draw a card unless that player pays {1} (hardcoded
//! base power = 1). A re-implementation with a dynamic-X cost gate is
//! future engine work.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Esper Sentinel");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .without_types(TypeLine::CREATURE.into()),
                    ),
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: draw_unless_opponent_pays,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_unless_opponent_pays(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Determine the casting opponent — the one who cast the noncreature spell.
    let Some(caster) = trig.triggering_caster() else { return Vec::new(); };
    // Compute X = this creature's power at resolution time.
    let x = script::power_of(state, trig.source).max(0) as u32;
    // GAP: OptionalPaymentKind::Mana requires a static ManaCost; dynamic X
    // (based on live power) is not expressible. Using a generic colorless cost
    // of X generic mana as a best-effort. The correct behavior would be a
    // dynamic mana gate that uses the creature's live power value.
    let cost_str = format!("{{{}}}", x);
    let mana_cost = ManaCost::parse(&cost_str).unwrap_or_else(|_| ManaCost::parse("{1}").unwrap());
    // "Draw a card unless that player pays {X}": the opponent is the chooser.
    // If they pay, no draw (then = no-op). If they don't pay, draw (else_effect).
    vec![Effect::OptionalPayment {
        chooser: caster,
        cost: OptionalPaymentKind::Mana(mana_cost),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::DrawCards {
            player: trig.controller,
            count: 1,
        })),
    }]
}
