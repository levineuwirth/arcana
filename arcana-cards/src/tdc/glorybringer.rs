//! Glorybringer — `{3}{R}{R}` 4/4 red Dragon with Flying and Haste.
//! "You may exert this creature as it attacks. When you do, it deals 4
//! damage to target non-Dragon creature an opponent controls."
//!
//! Flying/Haste are base keywords. Exert is the engine's attack-time
//! choice (CR 701.43d): exerted, Glorybringer stays tapped through its
//! controller's next untap step, and the "when you do" ability triggers
//! on that exert alone.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glorybringer");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars).with_exert_as_it_attacks().with_triggered_ability(
        TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::Custom(arcana_core::triggers::self_exerted),
            intervening_if: None,
            effect: exert_burn,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent)
                        .without_subtype_sym(dragon),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        },
    ))
}

fn exert_burn(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }]
}
