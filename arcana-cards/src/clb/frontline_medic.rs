//! Frontline Medic — `{2}{W}` 3/3 white Human Cleric.
//!
//! * Battalion — Whenever this creature and at least two other creatures
//!   attack, creatures you control gain indestructible until end of turn.
//! * Sacrifice this creature: Counter target spell with {X} in its mana
//!   cost unless its controller pays {3}.
//!
//! GAPs:
//! - The "battalion" gate (this + at least two OTHER attackers) has no
//!   matching `TriggerCondition`; we approximate with `SelfAttacks` and
//!   GAP the two-other-attackers requirement (the indestructible grant
//!   itself IS expressible via ForEach + GrantKeyword).
//! - The sacrifice ability is a counter-target-spell-unless-pay effect;
//!   `Effect::Counter` is not in the usable effect catalog for this class,
//!   so that ability is GAP'd entirely.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frontline Medic");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Battalion — approximated as SelfAttacks (see GAP).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: grant_indestructible,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Sacrifice: counter target spell unless pay {3} — GAP'd.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice Frontline Medic: Counter target spell with {X} in its mana cost unless its controller pays {3}.".into(),
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: counter_spell_gap,
            }),
    )
}

fn grant_indestructible(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "and at least two OTHER creatures attack" battalion gate is
    // unexpressible; we grant unconditionally on attack.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: arcana_core::objects::NULL_OBJECT_ID,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        }),
    }]
}

fn counter_spell_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: counter target spell unless its controller pays {3} —
    // `Effect::Counter` is not in the usable effect catalog for this class.
    Vec::new()
}
