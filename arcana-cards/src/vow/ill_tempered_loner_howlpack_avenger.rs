//! Ill-Tempered Loner // Howlpack Avenger — `{2}{R}{R}` red Creature — Human Werewolf / Werewolf.
//!
//! Front face (Ill-Tempered Loner): 3/3
//!   Whenever this creature is dealt damage, it deals that much damage to any target.
//!   {1}{R}: This creature gets +2/+0 until end of turn.
//!   Daybound (not modeled; see GAP below).
//!
//! Back face (Howlpack Avenger): (no P/T spec provided; using 5/5 as per oracle)
//!   Whenever a permanent you control is dealt damage, this creature deals that
//!   much damage to any target.
//!   {1}{R}: This creature gets +2/+0 until end of turn.
//!   Nightbound (not modeled; see GAP below).
//!
//! The back-face triggered ability ("Whenever a permanent you control is dealt
//! damage, this creature deals that much damage to any target") is wired via a
//! DamageDealt trigger (target_filter = a permanent you control) that forwards
//! `damage_amount()` to a chosen any-target, gated to the back face (face 1).
//!
//! GAP: Daybound/Nightbound keyword — day/night cycle not modeled in engine.
//! GAP: "Whenever this creature is dealt damage" (FRONT face) — the DamageDealt
//!      trigger's target_filter cannot pin the damage RECIPIENT to the source
//!      object itself (no self-referential TargetFilter for the damaged
//!      permanent); the front-face self-damage trigger is left unwired.
//! GAP: Transform conditions (day→night / night→day) not modeled; transform
//!      trigger not wired (no suitable TriggerCondition for day/night transitions).
//! The activated +2/+0 ability is modeled on the shared CardDefinition (works
//! for both faces, face_gate: None).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ill-Tempered Loner");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Daybound keyword not in engine keyword surface
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Howlpack Avenger");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            // GAP: Nightbound keyword not in engine keyword surface
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {1}{R}: +2/+0 until end of turn (shared by both faces via CardDefinition)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: This creature gets +2/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: pump_two_zero,
            })
            // Back face: "Whenever a permanent you control is dealt damage, this
            // creature deals that much damage to any target." DamageDealt trigger
            // (recipient = a permanent you control) forwarding the damage amount.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
                    ),
                    combat_only: false,
                },
                intervening_if: None,
                effect: reflect_damage_to_any_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // Back-face only.
            .with_trigger_face_gate(1, 1),
        // GAP: "Whenever this creature is dealt damage" (FRONT face) — no
        //      self-referential TargetFilter pins the damaged permanent to the
        //      source; front-face self-damage reflect left unwired.
        // GAP: Day/night transform triggers not modeled (no engine support for
        //      day/night cycle).
    )
}

fn reflect_damage_to_any_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let dmg_target = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: dmg_target,
        amount: n,
    }]
}

fn pump_two_zero(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
