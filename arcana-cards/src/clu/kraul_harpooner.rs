//! Kraul Harpooner — `{1}{G}` 3/2 Insect Warrior with Reach.
//! "Undergrowth — When this creature enters, choose up to one target creature
//! you don't control with flying. This creature gets +X/+0 until end of turn,
//! where X is the number of creature cards in your graveyard, then you may
//! have this creature fight that creature."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kraul Harpooner");
    let insect = reg.interner_mut().intern("Insect");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: undergrowth_pump_and_fight,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent)
                        .with_keyword(KeywordAbility::Flying),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn undergrowth_pump_and_fight(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // X = number of creature cards in your graveyard.
    let x = script::graveyard_matching(
        state,
        &ObjectFilter::creature(),
        trig.controller,
        trig.controller,
    ) as i32;
    let mut effects = vec![Effect::Pump {
        target: trig.source,
        power: x,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    // "up to one" — there may be no chosen target; only fight if one was chosen.
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::Fight {
            a: trig.source,
            b: *id,
        });
    }
    effects
}
