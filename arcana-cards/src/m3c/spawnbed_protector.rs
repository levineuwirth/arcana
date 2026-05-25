//! Spawnbed Protector — `{7}` 6/8 Eldrazi.
//! "At the beginning of your end step, return up to one target Eldrazi
//! creature card from your graveyard to your hand. Create two 1/1
//! colorless Eldrazi Scion creature tokens with 'Sacrifice this token:
//! Add {C}.'"

use arcana_core::effects::{Effect, TokenDefinition};
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
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spawnbed Protector");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let _scion = reg.interner_mut().intern("Scion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_end_step,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn on_end_step(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![];
    // Return target Eldrazi creature card from graveyard to hand.
    if let Some(target) = trig.targets.targets.first() {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
        }
    }
    // Create two 1/1 colorless Eldrazi Scion tokens.
    let scion = reg.interner().lookup("Scion").expect("Scion interned during register()");
    let eldrazi = reg.interner().lookup("Eldrazi").expect("Eldrazi interned during register()");
    let make_scion = || {
        let mut token_subtypes = SubtypeSet::default();
        token_subtypes.0.insert(eldrazi);
        token_subtypes.0.insert(scion);
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: scion,
                colors: ColorSet::colorless(),
                types: TypeLine::CREATURE.into(),
                subtypes: token_subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        }
    };
    effects.push(make_scion());
    effects.push(make_scion());
    effects
}
