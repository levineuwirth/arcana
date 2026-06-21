//! Whirlpool Warrior — `{2}{U}` 2/2 Merfolk Warrior.
//! When this creature enters, shuffle the cards from your hand into your
//! library, then draw that many cards.
//! {R}, Sacrifice this creature: Each player shuffles the cards from their
//! hand into their library, then draws that many cards.
//!
//! Both abilities are wired following the catalog Whirlpool Rider
//! precedent: "shuffle hand into library, then draw that many" is
//! approximated by counting current hand size and drawing that many (the
//! shuffle-hand-into-library half is a documented engine GAP — there is no
//! shuffle-hand-and-redraw effect). The activated ability does this per
//! player; its cost is {R} + sacrifice this creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Whirlpool Warrior");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_redraw_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}, Sacrifice this creature: Each player shuffles the cards from their hand into their library, then draws that many cards.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").unwrap(),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: each_player_redraw,
            }),
    )
}

fn etb_redraw_hand(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "shuffle hand into library" not modeled; counting hand size and
    // drawing that many (Whirlpool Rider precedent).
    let n = script::hand_size(state, trig.controller);
    vec![Effect::DrawCards {
        player: trig.controller,
        count: n,
    }]
}

fn each_player_redraw(
    state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "shuffle hand into library" not modeled; per player, count current
    // hand size and draw that many (Whirlpool Rider precedent).
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::DrawCards {
            player: p,
            count: script::hand_size(state, p),
        })
        .collect()
}
