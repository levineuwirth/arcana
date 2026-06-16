//! Tasha, the Witch Queen — `{3}{U}{B}` Legendary Planeswalker — Tasha, loyalty 5.
//!
//! Triggered: Whenever you cast a spell you don't own, create a 3/3 black Demon
//!   creature token.
//! +1: Draw a card. For each opponent, exile up to one target instant or sorcery
//!   card from that player's graveyard and put a page counter on it.
//! −3: You may cast a spell from among cards in exile with page counters on them
//!   without paying its mana cost.
//!
//! # Scope
//! GAP: the "cast a spell you don't own" trigger filter (ownership rather than
//!   controller) is not expressible — the Demon-token trigger is declared with
//!   a SpellCast condition but the ownership predicate is approximated by
//!   firing on any spell you cast; the effect creates the 3/3 Demon faithfully.
//!   (Marked Custom-unavailable; modeled as you-cast.)
//! GAP: the +1 graveyard-targeting "exile up to one instant/sorcery from each
//!   opponent's graveyard + page counter" needs an any-graveyard sentinel and a
//!   page-counter-tracked exile — only the "Draw a card" half is expressed.
//! GAP: −3 "cast a spell from exile with a page counter without paying its mana
//!   cost" — no exile-with-counter cast surface. Effect is Vec::new().

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::effects::TokenDefinition;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tasha, the Witch Queen");
    let tasha = reg.interner_mut().intern("Tasha");
    let _demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tasha);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
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
                // GAP: "spell you don't own" (ownership) is not expressible;
                // approximated as a spell you cast.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cast_demon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Draw a card. For each opponent, exile up to one target instant or sorcery card from that player's graveyard and put a page counter on it.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: You may cast a spell from among cards in exile with page counters on them without paying its mana cost.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_gap,
            }),
    )
}

fn cast_demon(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let demon = reg.interner().lookup("Demon").expect("Demon interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    let token = TokenDefinition {
        name: demon,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

fn plus_one_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: per-opponent graveyard exile + page counter not expressible;
    // only "Draw a card" is modeled.
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}

fn minus_three_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cast from exile (page-counter pool) without paying mana cost.
    Vec::new()
}
