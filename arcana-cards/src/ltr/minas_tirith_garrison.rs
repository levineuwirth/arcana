//! Minas Tirith Garrison — `{3}{U}` */5 Human Soldier.
//!
//! Oracle:
//! * "Minas Tirith Garrison's power is equal to the number of cards in your
//!   hand." — an ASYMMETRIC characteristic-defining ability (power `*`,
//!   toughness fixed 5) wired at Layer 7a via a SelfEntersBattlefield
//!   `self_pt_cda`: the compute reads the controller's hand size for power and
//!   returns the printed fixed toughness 5 on the non-`*` axis → `(hand, 5)`.
//! * "Whenever this creature attacks, you may tap any number of untapped Humans
//!   you control. Draw a card for each Human tapped this way." — the attack
//!   trigger is wired, but its effect is GAP'd: there is no "tap any number,
//!   then draw equal to the number tapped this way" primitive (no tap-any-
//!   number action that surfaces the count for the subsequent draw).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Minas Tirith Garrison");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // CDA — power = cards in your hand; toughness fixed 5 — resolved at 7a.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_tap_humans_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// Power = cards in your hand; toughness is the printed fixed 5.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let p = script::hand_size(s, who) as i32;
    (p, 5)
}

fn attack_tap_humans_draw(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "tap any number of untapped Humans you control, then draw a card
    //      for each tapped this way" — no tap-any-number primitive that
    //      surfaces the count for the subsequent draw.
    Vec::new()
}
