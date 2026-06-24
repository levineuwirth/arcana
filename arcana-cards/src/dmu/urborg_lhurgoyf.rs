//! Urborg Lhurgoyf — `{1}{G}` */1+* Lhurgoyf.
//!
//! Kicker {U} and/or {B}; as it enters, mill three cards for each time it was
//! kicked. Its power is equal to the number of creature cards in your
//! graveyard, and its toughness is that number plus 1.
//!
//! The CDA is wired at Layer 7a via a SelfEntersBattlefield self_pt_cda:
//! power = creature cards in your graveyard, toughness = power + 1 (bones
//! `Star` / `StarPlus(1)`). The Kicker cost and the "mill three per time it
//! was kicked" ETB are GAP'd — there is no kicker primitive nor a
//! times-kicked count to drive the mill amount.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urborg Lhurgoyf");
    let lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // CDA: power = creature cards in your graveyard; toughness = that + 1.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::StarPlus(1)),
        // GAP: "Kicker {U} and/or {B}" — kicker is not modeled.
        // GAP: "As this creature enters, mill three cards for each time it was
        // kicked" — no times-kicked count to drive the mill amount.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Layer 7a self-CDA: power = creature cards in your graveyard, toughness = +1.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            creature_cards_in_gy_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = creature cards in your graveyard; toughness = that number plus 1.
fn creature_cards_in_gy_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let filter = ObjectFilter::new().with_types(TypeLine::CREATURE.into());
    let n = script::graveyard_matching(s, &filter, who, who) as i32;
    (n, n + 1)
}
