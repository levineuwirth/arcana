//! Benalish Commander — `{3}{W}` */* Human Soldier.
//!
//! Oracle:
//! * "Benalish Commander's power and toughness are each equal to the number of
//!   Soldiers you control." — a characteristic-defining ability wired at Layer
//!   7a via a SelfEntersBattlefield `self_pt_from_match`: the count filter is
//!   "Soldiers you control" (built in the effect fn, which has `reg`, so it can
//!   name the Soldier subtype). SYMMETRIC — sets both P and T to the count.
//! * Suspend X—{X}{W}{W}. X can't be 0. — GAP: Suspend is not a supported
//!   `KeywordAbility` (alternative-cast keyword); `keywords: vec![]`.
//! * "Whenever a time counter is removed from this card while it's exiled,
//!   create a 1/1 white Soldier creature token." — GAP: there is no trigger
//!   condition for counter REMOVAL while exiled.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Benalish Commander");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // CDA — P/T each = number of Soldiers you control — resolved at 7a.
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

fn install_cda(_s: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Soldier")
        .controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match(
            trig.source,
            filter,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
