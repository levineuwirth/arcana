//! Shalai and Hallar — `{1}{R}{G}{W}` 3/3 Legendary Creature — Angel Elf.
//! Flying, vigilance.
//! Whenever one or more +1/+1 counters are put on a creature you control,
//! Shalai and Hallar deals that much damage to target opponent.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shalai and Hallar");
    let angel = reg.interner_mut().intern("Angel");
    let elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(elf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CounterAdded {
                on: TriggerSelf::AnyMatching(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                kind: Some(CounterKind::PlusOnePlusOne),
                chapter: None,
            },
            intervening_if: None,
            effect: deal_counter_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::Exactly(1),
                controller: Some(ControllerConstraint::Opponent),
            }],
        }),
    )
}

fn deal_counter_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "deals THAT MUCH damage" where "that much" = the number of +1/+1
    // counters just added. PendingTrigger exposes no counter-amount accessor
    // and with_trigger_dynamic_x cannot read it either, so the dynamic damage
    // amount is not computable. Per the dynamic-amount rule, GAP the whole
    // effect rather than emit a wrong literal.
    Vec::new()
}
