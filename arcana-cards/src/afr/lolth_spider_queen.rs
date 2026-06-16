//! Lolth, Spider Queen — `{3}{B}{B}` Legendary Planeswalker — Lolth, starting
//! loyalty 4. Mono-black.
//!
//! Trigger: Whenever a creature you control dies, put a loyalty counter on Lolth
//!   (ZoneChange creature-you-control battlefield → graveyard → AddCounters).
//! 0: You draw a card and you lose 1 life.
//! −3: Create two 2/1 black Spider creature tokens with menace and reach.
//! −8: emblem ("Whenever an opponent is dealt combat damage by one or more
//!   creatures you control, if that player lost less than 8 life this turn, they
//!   lose life equal to the difference."). GAP: this is a per-turn-life-loss
//!   threshold difference with a "one or more creatures" aggregation — not
//!   expressible. Triggered emblem shell declared (combat damage to opponent);
//!   effect GAP'd.

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lolth, Spider Queen");
    let lolth = reg.interner_mut().intern("Lolth");
    let _spider = reg.interner_mut().intern("Spider");
    let _emblem = reg.interner_mut().intern("Lolth, Spider Queen emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lolth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: creature_dies_loyalty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: You draw a card and you lose 1 life.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_draw_lose,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Create two 2/1 black Spider creature tokens with \
                       menace and reach.".into(),
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
                effect: minus_three_spiders,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"Whenever an opponent is \
                       dealt combat damage by one or more creatures you \
                       control, if that player lost less than 8 life this turn, \
                       they lose life equal to the difference.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

fn spider_token(reg: &CardRegistry) -> TokenDefinition {
    let spider = reg.interner().lookup("Spider").expect("Spider interned");
    let mut st = SubtypeSet::default();
    st.0.insert(spider);
    TokenDefinition {
        name: spider,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: st,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Reach],
        abilities: vec![],
    }
}

fn creature_dies_loyalty(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Loyalty, count: 1 }]
}

fn zero_draw_lose(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::LoseLife { player: ctx.controller, amount: 1 },
    ]
}

fn minus_three_spiders(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::CreateToken { controller: ctx.controller, token: spider_token(reg) },
        Effect::CreateToken { controller: ctx.controller, token: spider_token(reg) },
    ]
}

fn minus_eight_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Lolth, Spider Queen emblem").expect("emblem interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: emblem_difference_gap,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_difference_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if that player lost less than 8 life this turn, they lose life equal
    //      to the difference" — per-turn-life-loss threshold + dynamic difference
    //      is not expressible.
    Vec::new()
}
