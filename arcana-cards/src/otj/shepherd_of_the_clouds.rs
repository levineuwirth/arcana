//! Shepherd of the Clouds — `{4}{W}` 4/3 Creature — Pegasus.
//!
//! Oracle:
//! * Flying, vigilance.
//! * When this creature enters, return target permanent card with mana value 3
//!   or less from your graveyard to your hand. Return that card to the
//!   battlefield instead if you control a Mount.

use arcana_core::effects::{Condition, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shepherd of the Clouds");
    let pegasus = reg.interner_mut().intern("Pegasus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pegasus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_recur,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::permanent().with_max_cmc(3),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn etb_recur(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let id = *id;
    // "Return that card to the battlefield instead if you control a Mount."
    let mount = script::subtype_filter(reg, "Mount").controlled_by(ControllerConstraint::You);
    vec![Effect::Conditional {
        condition: Condition::ControlPermanentMatching(mount),
        then: Box::new(Effect::ReturnFromGraveyardToBattlefield { target: id }),
        otherwise: Some(Box::new(Effect::ReturnFromGraveyardToHand { target: id })),
    }]
}
