//! The Fifteenth Doctor — `{2}{U}{R}` 3/3 Legendary Time Lord Doctor.
//! "Whenever The Fifteenth Doctor enters or attacks, mill three cards.
//! You may put an artifact card with mana value 2 or 3 from among them
//! into your hand."
//!
//! The "enters or attacks" clause is decomposed into two triggers
//! sharing one resolver, modeled as DigTopN over the top three cards:
//! you may take one artifact card with mana value 2 or 3 (a 2..=3 CMC
//! range filter) into your hand; the rest go to the graveyard — which
//! reproduces "mill three, you may put a matching artifact into hand."
//!
//! GAP (static): "The first nonartifact spell you cast each turn has
//! improvise." — a cost-modification static with no expressible
//! triggered/activated primitive.
//!
//! ("Mill" is reminder text, not a real KeywordAbility variant.)

use arcana_core::effects::{DigRest, Effect};
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
    let name = reg.interner_mut().intern("The Fifteenth Doctor");
    let time_lord = reg.interner_mut().intern("Time Lord");
    let doctor = reg.interner_mut().intern("Doctor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(time_lord);
    subtypes.0.insert(doctor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
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
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: mill_take_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: mill_take_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn mill_take_artifact(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 3,
        filter: Some(
            ObjectFilter::new()
                .with_types(TypeLine::ARTIFACT.into())
                .with_min_cmc(2)
                .with_max_cmc(3),
        ),
        rest: DigRest::Graveyard,
    }]
}
