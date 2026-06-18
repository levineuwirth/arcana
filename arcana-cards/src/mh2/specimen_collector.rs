//! Specimen Collector — `{4}{U}` 2/1 Vedalken Wizard.
//! ETB: create a 1/1 green Squirrel and a 0/3 blue Crab.
//! Dies: create a token that's a copy of target token you control.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Specimen Collector");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let wizard = reg.interner_mut().intern("Wizard");
    let _squirrel = reg.interner_mut().intern("Squirrel");
    let _crab = reg.interner_mut().intern("Crab");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: copy_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You)
                            .tokens_only(),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_tokens(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let squirrel = reg.interner().lookup("Squirrel").unwrap_or_default();
    let crab = reg.interner().lookup("Crab").unwrap_or_default();
    let mut sq_subtypes = SubtypeSet::default();
    sq_subtypes.0.insert(squirrel);
    let mut cr_subtypes = SubtypeSet::default();
    cr_subtypes.0.insert(crab);
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: squirrel,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: sq_subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        },
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: crab,
                colors: ColorSet::blue(),
                types: TypeLine::CREATURE.into(),
                subtypes: cr_subtypes,
                power: Some(PtValue::Fixed(0)),
                toughness: Some(PtValue::Fixed(3)),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}

fn copy_token(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::CopyPermanent { target: *id }]
}
