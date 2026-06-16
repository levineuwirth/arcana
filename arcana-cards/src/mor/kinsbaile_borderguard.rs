//! Kinsbaile Borderguard — `{1}{W}{W}` 1/1 Kithkin Soldier.
//! Enters with a +1/+1 counter for each other Kithkin you control.
//! When it dies, create a 1/1 white Kithkin Soldier token for each counter on it.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kinsbaile Borderguard");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_make_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_counters(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // "+1/+1 counter for each OTHER Kithkin you control" — count Kithkin you
    // control and subtract 1 for this creature (it is itself a Kithkin).
    let total = script::count_matching(
        state,
        &script::subtype_filter(reg, "Kithkin")
            .controlled_by(arcana_core::targets::ControllerConstraint::You),
        trig.controller,
    );
    let n = total.saturating_sub(1);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: n,
    }]
}

fn dies_make_tokens(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // "create a 1/1 white Kithkin Soldier token for each counter on it" — read
    // the +1/+1 counter count off the dying object.
    let id = trig.dying_object().unwrap_or(trig.source);
    let n = state
        .objects
        .get(id)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    if n == 0 {
        return Vec::new();
    }
    let kithkin = match reg.interner().lookup("Kithkin") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let soldier = reg.interner().lookup("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    if let Some(sol) = soldier {
        subtypes.0.insert(sol);
    }
    let token = TokenDefinition {
        name: kithkin,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        })
        .collect()
}
