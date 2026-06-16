//! Vraska, Swarm's Eminence — `{2}{B/G}{B/G}` Legendary Planeswalker —
//! Vraska, starting loyalty 4.
//!
//! Whenever a creature you control with deathtouch deals damage to a player or
//!   planeswalker, put a +1/+1 counter on that creature.
//! −2: Create a 1/1 black Assassin creature token with deathtouch and
//!   "Whenever this token deals damage to a planeswalker, destroy that
//!   planeswalker."
//!
//! Note: the static trigger's "to a player or planeswalker" is approximated
//!   with TargetFilter::AnyTarget (creature/player/planeswalker) — the
//!   demonstrated TargetFilter surface has no exact "player or planeswalker"
//!   shape. The +1/+1 counter is placed on the damage source extracted from
//!   the firing event.
//! Note: the token's rider "deals damage to a planeswalker, destroy that
//!   planeswalker" is modeled with a DamageDealt trigger whose target filter
//!   is the planeswalker permanent; it destroys the damage target.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::events::{DamageTarget, GameEvent};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetFilter,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vraska, Swarm's Eminence");
    let vraska = reg.interner_mut().intern("Vraska");
    let _assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vraska);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B/G}{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    let deathtouch_you = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_keyword(KeywordAbility::Deathtouch);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: deathtouch_you,
                    target_filter: TargetFilter::AnyTarget,
                    combat_only: false,
                },
                intervening_if: None,
                effect: static_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Create a 1/1 black Assassin creature token with \
                       deathtouch and \"Whenever this token deals damage to a \
                       planeswalker, destroy that planeswalker.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_token,
            }),
    )
}

fn static_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let GameEvent::DamageDealt { source, .. } = &trig.trigger_event else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn minus_two_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let assassin = reg.interner().lookup("Assassin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(assassin);
    let token = TokenDefinition {
        name: assassin,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        abilities: vec![TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .with_types(TypeLine::PLANESWALKER.into()),
                ),
                combat_only: false,
            },
            intervening_if: None,
            effect: token_destroy_pw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![],
        }],
    };
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token,
    }]
}

fn token_destroy_pw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let GameEvent::DamageDealt { target, .. } = &trig.trigger_event else {
        return Vec::new();
    };
    match target {
        DamageTarget::Object(id) => vec![Effect::DestroyPermanent { target: *id }],
        DamageTarget::Player(_) => Vec::new(),
    }
}
