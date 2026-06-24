//! Kiyomaro, First to Stand — `{3}{W}{W}` */* Legendary Spirit.
//!
//! Oracle:
//! * Kiyomaro's power and toughness are each equal to the number of cards in
//!   your hand — a characteristic-defining ability wired at Layer 7a via a
//!   `SelfEntersBattlefield` `self_pt_cda`. Both `*` axes resolve to the
//!   controller's hand size.
//! * GAP: "As long as you have four or more cards in hand, Kiyomaro has
//!   vigilance." — a conditional static keyword grant (no trigger/cost); the
//!   "as long as" continuous gate is outside the demonstrated surface.
//! * Whenever Kiyomaro deals damage, if you have seven or more cards in hand,
//!   you gain 7 life. — a `DamageDealt` trigger scoped to this object (by name)
//!   with an intervening-if on hand size.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kiyomaro, First to Stand");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let self_name = reg.interner().lookup("Kiyomaro, First to Stand");
    let self_filter = ObjectFilter { name: self_name, ..ObjectFilter::default() };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // `*/*` — both axes are a CDA: cards in your hand. Resolved at Layer 7a
        // by install_cda.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
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
            // Whenever Kiyomaro deals damage, if you have 7+ cards, gain 7 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: self_filter,
                    target_filter: TargetFilter::AnyTarget,
                    combat_only: false,
                },
                intervening_if: Some(if_seven_or_more_cards),
                effect: gain_seven_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Layer 7a self-CDA: P/T each equal to the number of cards in your hand.
fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// P/T = the number of cards in your hand.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s.objects.objects_in_zone(Zone::Hand(who)).count() as i32;
    (n, n)
}

/// Intervening-if: you have seven or more cards in hand.
fn if_seven_or_more_cards(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::hand_at_least(s, you, 7)
}

fn gain_seven_life(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife { player: trig.controller, amount: 7 }]
}
