//! Aeon Chronicler — `{3}{U}{U}` */* Avatar.
//!
//! Oracle:
//! * Aeon Chronicler's power and toughness are each equal to the number of
//!   cards in your hand. (Installed at Layer 7a via an ETB self-CDA —
//!   `self_pt_cda` reading the controller's hand size; symmetric `*`/`*`.)
//! * Suspend X—{X}{3}{U}. X can't be 0. — GAP: "Suspend" is not a usable
//!   `KeywordAbility` variant (the suspend cast / time-counter mechanic is
//!   unmodeled).
//! * Whenever a time counter is removed from this card while it's exiled,
//!   draw a card. — GAP: no counter-removed / exile-zone time-counter
//!   `TriggerCondition` variant.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aeon Chronicler");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
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

fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cards_in_hand,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn cards_in_hand(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s.objects.objects_in_zone(Zone::Hand(who)).count() as i32;
    (n, n)
}
