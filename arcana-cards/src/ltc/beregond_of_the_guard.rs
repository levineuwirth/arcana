//! Beregond of the Guard — `{3}{W}` 3/3 Legendary Human Soldier.
//! "Whenever Beregond or another Human you control enters, creatures
//! you control get +1/+1 and gain vigilance until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Beregond of the Guard");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: script::subtype_filter(reg, "Human")
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_human_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_human_enters(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &arcana_core::targets::ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let mut effects: Vec<Effect> = ids
        .iter()
        .map(|&id| Effect::Pump {
            target: id,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        })
        .collect();
    for &id in &ids {
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}
