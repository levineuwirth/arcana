//! Sarkhan the Masterless — `{3}{R}{R}` Legendary Planeswalker — Sarkhan, starting loyalty 5.
//!
//! Triggered (static): "Whenever a creature attacks you or a planeswalker you
//!   control, each Dragon you control deals 1 damage to that creature." Modeled as
//!   a `CreatureAttacks` triggered ability; the resolver reads the attacker from
//!   the event and has each Dragon you control deal 1 damage to it. GAP: the
//!   "attacks you or a planeswalker you control" defending restriction is not
//!   filtered (fires on any attacker) — an approximation.
//! +1: Until end of turn, each planeswalker you control becomes a 4/4 red Dragon
//!   creature and gains flying. GAP: animating planeswalkers into 4/4 fliers
//!   (type-change + base-P/T + keyword on a set of permanents) is not expressible;
//!   shell declared at the correct +1 cost.
//! −3: Create a 4/4 red Dragon creature token with flying.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::events::{DamageTarget, GameEvent};
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
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sarkhan the Masterless");
    let sarkhan = reg.interner_mut().intern("Sarkhan");
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sarkhan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::new(),
                },
                intervening_if: None,
                effect: on_attack_dragons_ping,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until end of turn, each planeswalker you control \
                       becomes a 4/4 red Dragon creature and gains flying.".into(),
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
                effect: plus_one_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Create a 4/4 red Dragon creature token with flying.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_dragon,
            }),
    )
}

fn on_attack_dragons_ping(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let GameEvent::CreatureAttacks { attacker, .. } = trig.trigger_event else {
        return Vec::new();
    };
    let dragon = match reg.interner().lookup("Dragon") {
        Some(d) => d,
        None => return Vec::new(),
    };
    let dragons = script::ids_matching(
        state,
        &ObjectFilter::new()
            .with_subtype_sym(dragon)
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    dragons
        .into_iter()
        .map(|d| Effect::DealDamage {
            source: d,
            target: DamageTarget::Object(attacker),
            amount: 1,
        })
        .collect()
}

fn plus_one_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: animate each planeswalker you control into a 4/4 red Dragon with
    //      flying (type-change + base-P/T + keyword over a set) not expressible.
    Vec::new()
}

fn minus_three_dragon(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = reg.interner().lookup("Dragon").expect("Dragon interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(dragon);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: dragon,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
