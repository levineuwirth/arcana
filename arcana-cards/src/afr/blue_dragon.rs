//! Blue Dragon — `{5}{U}{U}` 5/5 blue Dragon with Flying.
//! "Lightning Breath — When this creature enters, until your next turn,
//! target creature an opponent controls gets -3/-0, up to one other
//! target creature gets -2/-0, and up to one other target creature gets
//! -1/-0."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blue Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: lightning_breath,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
            ],
        }),
    )
}

fn lightning_breath(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let dur = Duration::UntilYourNextTurn(trig.controller);
    let mut out = Vec::new();
    let mut iter = trig.targets.targets.iter();
    if let Some(TargetChoice::Object(id)) = iter.next() {
        out.push(Effect::Pump {
            target: *id,
            power: -3,
            toughness: 0,
            duration: dur,
            keywords: vec![],
        });
    }
    if let Some(TargetChoice::Object(id)) = iter.next() {
        out.push(Effect::Pump {
            target: *id,
            power: -2,
            toughness: 0,
            duration: dur,
            keywords: vec![],
        });
    }
    if let Some(TargetChoice::Object(id)) = iter.next() {
        out.push(Effect::Pump {
            target: *id,
            power: -1,
            toughness: 0,
            duration: dur,
            keywords: vec![],
        });
    }
    out
}
