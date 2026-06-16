//! Kaito, Cunning Infiltrator — `{1}{U}{U}` Legendary Planeswalker — Kaito,
//! starting loyalty 3. Mono-blue.
//!
//! Static-ish trigger: Whenever a creature you control deals combat damage to a
//!   player, put a loyalty counter on Kaito (`DamageDealt` combat trigger →
//!   AddCounters(Loyalty) on self).
//! +1: Up to one target creature you control can't be blocked this turn. Draw a
//!   card, then discard a card. (`CantBeBlocked` EndOfTurn + draw + discard.)
//! −2: Create a 2/1 blue Ninja creature token.
//! −9: emblem ("Whenever a player casts a spell, you create a 2/1 blue Ninja
//!   creature token."). Triggered emblem: SpellCast(Any) → create Ninja token.

use arcana_core::effects::{Effect, EmblemDefinition, TokenDefinition, DiscardChoice};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaito, Cunning Infiltrator");
    let kaito = reg.interner_mut().intern("Kaito");
    let _ninja = reg.interner_mut().intern("Ninja");
    let _emblem = reg.interner_mut().intern("Kaito, Cunning Infiltrator emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kaito);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_loyalty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature you control can't be \
                       blocked this turn. Draw a card, then discard a \
                       card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_unblockable,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Create a 2/1 blue Ninja creature token.".into(),
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
                effect: minus_two_ninja,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-9: You get an emblem with \"Whenever a player casts a \
                       spell, you create a 2/1 blue Ninja creature \
                       token.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_emblem,
            }),
    )
}

fn ninja_token(reg: &CardRegistry) -> TokenDefinition {
    let ninja = reg.interner().lookup("Ninja").expect("Ninja interned");
    let mut st = SubtypeSet::default();
    st.0.insert(ninja);
    TokenDefinition {
        name: ninja,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: st,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    }
}

fn combat_damage_loyalty(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Loyalty,
        count: 1,
    }]
}

fn plus_one_unblockable(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out = Vec::new();
    if let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() {
        out.push(Effect::CantBeBlocked { target: *id, duration: Duration::EndOfTurn });
    }
    out.push(Effect::DrawCards { player: ctx.controller, count: 1 });
    out.push(Effect::Discard { player: ctx.controller, count: 1, choice: DiscardChoice::ControllerChooses });
    out
}

fn minus_two_ninja(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateToken { controller: ctx.controller, token: ninja_token(reg) }]
}

fn minus_nine_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Kaito, Cunning Infiltrator emblem").expect("emblem interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: emblem_make_ninja,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_make_ninja(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateToken { controller: trig.controller, token: ninja_token(reg) }]
}
