//! Burning-Yard Trainer — `{4}{R}` 3/3 Creature — Human Knight (red).
//!
//! * Trample, haste.
//! * "When this creature enters, another target Knight you control gets +2/+2
//!   and gains trample and haste until end of turn." — SelfEnters trigger
//!   pumping a target Knight you control by +2/+2 with Trample and Haste.
//!   (The "another" self-exclusion is a minor fidelity gap.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Burning-Yard Trainer");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    let knight_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(knight);

    reg.register(CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::SelfEntersBattlefield,
        intervening_if: None,
        effect: pump_knight,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Permanent(knight_filter),
            count: TargetCount::Exactly(1),
            controller: None,
        }],
    }))
}

fn pump_knight(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
    }]
}
