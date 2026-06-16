//! Narset of the Ancient Way — `{1}{U}{R}{W}` Legendary Planeswalker — Narset, starting loyalty 5.
//!
//! +1: You gain 2 life. Add {U}, {R}, or {W} (spend only on noncreature
//!   spells). Modeled as `GainLife(2)` + `AddMana` (one blue mana as a
//!   stand-in for the U/R/W choice); the spend restriction is GAP'd.
//! −2: Draw a card, then you may discard a card. When you discard a
//!   nonland card this way, Narset deals damage equal to that card's
//!   mana value to target creature or planeswalker. Modeled as draw +
//!   discard; the conditional dynamic-X damage rider is GAP'd.
//! −6: You get an emblem with "Whenever you cast a noncreature spell,
//!   this emblem deals 2 damage to any target." Fully implemented as a
//!   triggered emblem.

use arcana_core::effects::{DiscardChoice, Effect, EmblemDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Narset of the Ancient Way");
    let narset = reg.interner_mut().intern("Narset");
    let _emblem = reg.interner_mut().intern("Narset of the Ancient Way emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(narset);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You gain 2 life. Add {U}, {R}, or {W}. Spend this \
                       mana only to cast a noncreature spell.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_life_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Draw a card, then you may discard a card. When you \
                       discard a nonland card this way, Narset deals damage \
                       equal to that card's mana value to target creature or \
                       planeswalker.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"Whenever you cast a \
                       noncreature spell, this emblem deals 2 damage to any \
                       target.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

fn plus_one_life_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: U/R/W choice + "spend only on noncreature" restriction not
    // expressible; gain 2 life and add one blue mana as a stand-in.
    vec![
        Effect::GainLife { player: ctx.controller, amount: 2 },
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
        },
    ]
}

fn minus_two_loot(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Draw, then discard. GAP: the "when you discard a nonland card, deal
    // its mana value to a target creature or planeswalker" reflexive
    // dynamic-X rider isn't expressible.
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

fn minus_six_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Narset of the Ancient Way emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        not_types: Some(TypeLine::CREATURE.into()),
                        ..Default::default()
                    }),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_burn,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }],
        },
    }]
}

fn emblem_burn(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let dt = match trig.targets.targets.first() {
        Some(TargetChoice::Object(id)) => DamageTarget::Object(*id),
        Some(TargetChoice::Player(p)) => DamageTarget::Player(*p),
        Some(TargetChoice::ObjectOrPlayer(o)) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: dt,
        amount: 2,
    }]
}
