//! Adéwalé, Breaker of Chains — `{1}{U}{B}` 4/1 Legendary Human Assassin
//! Pirate (black/blue).
//!
//! Oracle:
//! * When Adéwalé enters, reveal the top six cards of your library. Put an
//!   Assassin, Pirate, or Vehicle card from among them into your hand and the
//!   rest on the bottom of your library in a random order.
//! * Whenever a Vehicle you control deals combat damage to a player, you may
//!   return this card from your graveyard to your hand.
//!
//! The ETB is modeled with `Effect::DigTopN` (look at top six, may take a
//! matching card to hand, rest to bottom in random order) — a faithful fit,
//! with the subtype-OR filter (Assassin / Pirate / Vehicle) built at
//! resolution time. The graveyard-return trigger fires on a Vehicle you
//! control dealing combat damage to a player; the "may" is a resolution
//! fidelity gap (the return is unconditional).

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Adéwalé, Breaker of Chains");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let pirate = reg.interner_mut().intern("Pirate");
    let _vehicle = reg.interner_mut().intern("Vehicle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);
    subtypes.0.insert(pirate);

    let vehicle_filter = script::subtype_filter(reg, "Vehicle")
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig_six,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: vehicle_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: return_self_from_graveyard,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_dig_six(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let syms: Vec<_> = ["Assassin", "Pirate", "Vehicle"]
        .iter()
        .filter_map(|n| reg.interner().lookup(n))
        .collect();
    let filter = ObjectFilter::default().with_subtypes_any(syms);
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 6,
        filter: Some(filter),
        rest: DigRest::BottomRandom,
    }]
}

fn return_self_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "may" return is a resolution fidelity gap (return is unconditional).
    vec![Effect::ReturnFromGraveyardToHand { target: trig.source }]
}
