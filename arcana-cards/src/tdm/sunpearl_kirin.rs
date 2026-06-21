//! Sunpearl Kirin — `{1}{W}` 2/1 Kirin.
//!
//! Oracle:
//! * Flash
//! * Flying
//! * When this creature enters, return up to one other target nonland
//!   permanent you control to its owner's hand. If it was a token, draw a
//!   card.
//!
//! Flash + Flying wired. The ETB bounce of an up-to-one nonland permanent
//! you control is wired; the "if it was a token, draw a card" rider is
//! GAP'd (no resolution-time was-a-token test once the permanent has left
//! the battlefield). The "other" self-exclusion is a minor documented
//! over-permissiveness.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Sunpearl Kirin");
    let kirin = reg.interner_mut().intern("Kirin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kirin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: bounce_target,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::You)
                        .without_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn bounce_target(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "If it was a token, draw a card." — no resolution-time
    //      was-a-token test on the bounced permanent.
    vec![Effect::ReturnToHand { target: *id }]
}
