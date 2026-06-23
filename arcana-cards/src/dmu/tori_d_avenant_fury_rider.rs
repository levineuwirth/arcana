//! Tori D'Avenant, Fury Rider — `{1}{R}{R}{W}` 3/3 Legendary Human Knight.
//! Vigilance, trample.
//! Whenever Tori attacks: all OTHER attacking creatures you control get
//! +1/+1 until end of turn; other red attacking creatures you control
//! gain trample until end of turn; untap each other white attacking
//! creature you control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tori D'Avenant, Fury Rider");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attack(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let attackers = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .attacking_only();

    // All OTHER attacking creatures you control get +1/+1.
    let pump_ids: Vec<_> = script::ids_matching(state, &attackers, trig.controller)
        .into_iter()
        .filter(|id| *id != trig.source)
        .collect();

    // Other RED attacking creatures gain trample.
    let red_attackers = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .attacking_only()
        .with_colors(ColorSet::red());
    let trample_ids: Vec<_> = script::ids_matching(state, &red_attackers, trig.controller)
        .into_iter()
        .filter(|id| *id != trig.source)
        .collect();

    // Untap each other WHITE attacking creature.
    let white_attackers = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .attacking_only()
        .with_colors(ColorSet::white());
    let untap_ids: Vec<_> = script::ids_matching(state, &white_attackers, trig.controller)
        .into_iter()
        .filter(|id| *id != trig.source)
        .collect();

    vec![Effect::Sequence(vec![
        Effect::ForEach {
            targets: pump_ids,
            effect: Box::new(Effect::Pump {
                target: NULL_OBJECT_ID,
                power: 1,
                toughness: 1,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
        },
        Effect::ForEach {
            targets: trample_ids,
            effect: Box::new(Effect::GrantKeyword {
                target: NULL_OBJECT_ID,
                keyword: KeywordAbility::Trample,
                duration: Duration::EndOfTurn,
            }),
        },
        Effect::ForEach {
            targets: untap_ids,
            effect: Box::new(Effect::Untap {
                target: NULL_OBJECT_ID,
            }),
        },
    ])]
}
