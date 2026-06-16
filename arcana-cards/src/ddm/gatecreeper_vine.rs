//! Gatecreeper Vine — `{1}{G}` 0/2 Plant with Defender.
//! "When this creature enters, you may search your library for a basic
//! land card or a Gate card, reveal it, put it into your hand, then
//! shuffle."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gatecreeper Vine");
    let plant = reg.interner_mut().intern("Plant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_tutor,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// ETB: search for a basic land card and put it into your hand.
/// GAP: oracle is "a basic land card OR a Gate card" — a single
/// `ObjectFilter` can't express that disjunction; emitting the
/// basic-land half (TutorToHand's "may" pick handles the optional).
fn etb_tutor(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::permanent()
            .with_types(TypeLine::LAND.into())
            .with_supertypes(SupertypeSet(SupertypeSet::BASIC)),
        reveal: true,
    }]
}
