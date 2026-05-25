//! Ashe, Princess of Dalmasca — `{2}{W}` 3/2 white Legendary Creature — Human Rebel Noble.
//! "Whenever Ashe attacks, look at the top five cards of your library. You may reveal an
//! artifact card from among them and put it into your hand. Put the rest on the bottom of
//! your library in a random order."
//!
//! # Notes
//! GAP: look at top 5, selectively reveal artifact to hand, rest to bottom in random order —
//! no exact Effect variant. Using TutorToHand with artifact filter as best approximation
//! (the "look at top 5 then choose" selection is not expressible, normal tutor searches library).

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Ashe, Princess of Dalmasca");
    let human = reg.interner_mut().intern("Human");
    let rebel = reg.interner_mut().intern("Rebel");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rebel);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_find_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_find_artifact(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: look at top 5 cards specifically then choose from among them — using TutorToHand
    // with artifact filter as best approximation (normal library search).
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        reveal: true,
    }]
}
