//! Syr Konrad, the Grim — `{3}{B}{B}` 5/4 Legendary Human Knight.
//!
//! * Whenever another creature dies, or a creature card is put into a
//!   graveyard from anywhere other than the battlefield, or a creature card
//!   leaves your graveyard, Syr Konrad deals 1 damage to each opponent.
//! * {1}{B}: Each player mills a card.
//!
//! The Scryfall-parsed "Mill" keyword is not a real KeywordAbility — the card
//! has no keyword line — so `keywords: vec![]`. The damage trigger has three
//! OR'd death/graveyard conditions; only the first ("another creature dies")
//! is wired via `ZoneChange` (creature: battlefield → graveyard). The other
//! two clauses (a creature card entering a graveyard from a non-battlefield
//! zone; a creature card leaving your graveyard) are GAP'd — a single
//! `TriggerCondition` can't express the disjunction.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Syr Konrad, the Grim");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "Whenever another creature dies" (first OR clause).
                // GAP: the other two clauses (creature card to a graveyard
                //       from a non-battlefield zone; creature card leaving
                //       your graveyard) can't be OR'd into one condition.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: damage_each_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}: Each player mills a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: each_player_mills,
            }),
    )
}

fn damage_each_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(p),
            amount: 1,
        })
        .collect()
}

fn each_player_mills(
    state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 1 })
        .collect()
}
