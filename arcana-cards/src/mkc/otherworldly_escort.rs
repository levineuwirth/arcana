//! Otherworldly Escort — `{3}{W}` 4/3 Human Detective.
//! "Flash.
//!  When this creature dies, if it's not a Spirit, return it to the battlefield
//!  under its owner's control with four charge counters on it. It's a Spirit
//!  Detective.
//!  {1}{W}, {T}, Remove a charge counter from this creature: Destroy target
//!  creature that dealt damage to you this turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Otherworldly Escort");
    let human = reg.interner_mut().intern("Human");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(detective);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                // GAP: intervening-if "if it's not a Spirit" — no conditions::
                // predicate for "source is not subtype X"; fires unconditionally.
                intervening_if: None,
                effect: dies_return_with_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{W}, {T}, Remove a charge counter from this creature: Destroy target creature that dealt damage to you this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                    tap: true,
                    remove_self_counter: Some((CounterKind::Charge, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_target_creature,
            }),
    )
}

fn dies_return_with_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    // GAP: "It's a Spirit Detective / no longer a Human" type swap is not
    // expressible on a returning permanent (no post-return type override).
    vec![Effect::Sequence(vec![
        Effect::ReturnFromGraveyardToBattlefield { target: id },
        Effect::AddCounters {
            target: id,
            kind: CounterKind::Charge,
            count: 4,
        },
    ])]
}

fn destroy_target_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that dealt damage to you this turn" restriction has no TargetFilter;
    // targets any creature.
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}
