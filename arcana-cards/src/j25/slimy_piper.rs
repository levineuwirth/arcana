//! Slimy Piper — `{1}{G}` 2/1 green creature (Fungus Bard).
//! "Whenever this creature attacks, it gets +1/+1 until end of turn. If you
//! control four or more creatures, it gets +2/+2 and gains indestructible
//! until end of turn instead."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slimy Piper");
    let fungus = reg.interner_mut().intern("Fungus");
    let bard = reg.interner_mut().intern("Bard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    subtypes.0.insert(bard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    if n >= 4 {
        vec![
            Effect::Pump {
                target: trig.source,
                power: 2,
                toughness: 2,
                duration: Duration::EndOfTurn,
                keywords: vec![KeywordAbility::Indestructible],
            },
        ]
    } else {
        vec![
            Effect::Pump {
                target: trig.source,
                power: 1,
                toughness: 1,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            },
        ]
    }
}
