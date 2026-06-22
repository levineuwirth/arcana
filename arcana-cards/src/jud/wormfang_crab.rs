//! Wormfang Crab — `{3}{U}` 3/6 Nightmare Crab.
//! "This creature can't be blocked." (self-static, modeled on ETB.)
//! "When this creature enters, an opponent chooses a permanent you
//!  control other than this creature and exiles it."
//! "When this creature leaves the battlefield, return the exiled card
//!  to the battlefield under its owner's control."
//! The exile+auto-return-on-leave is modeled with
//! Effect::ExileUntilSourceLeaves (the "opponent chooses" picker is a
//! fidelity gap — a permanent you control other than this is chosen
//! deterministically; the leave-return linkage is exact).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wormfang Crab");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let crab = reg.interner_mut().intern("Crab");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    subtypes.0.insert(crab);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_unblockable,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_own,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_unblockable(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::WhileSourceOnBattlefield,
    }]
}

fn etb_exile_own(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "a permanent you control other than this creature" — chosen
    // deterministically (the opponent-chooses picker is a fidelity gap).
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let Some(target) = ids.into_iter().find(|&id| id != trig.source) else {
        return Vec::new();
    };
    // ExileUntilSourceLeaves auto-returns the card when Wormfang Crab leaves,
    // covering the third (leave-the-battlefield) ability.
    vec![Effect::ExileUntilSourceLeaves {
        source: trig.source,
        target,
    }]
}
