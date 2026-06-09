//! Mwonvuli Beast Tracker — `{1}{G}{G}` 2/1 green Human Scout.
//! Keywords: Reach, Hexproof (printed on card).
//! "When this creature enters, search your library for a creature card with deathtouch,
//! hexproof, reach, or trample and reveal it. Shuffle and put that card on top."
//! Keyword disjunction modeled via `ObjectFilter::with_keywords_any` (library cards
//! match by base characteristics, which is correct for a search).
//! GAP: "put that card on top" — TutorToHand puts it in hand instead.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mwonvuli Beast Tracker");
    let human = reg.interner_mut().intern("Human");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Hexproof],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put that card on top" — TutorToHand puts the card in hand instead.
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::creature().with_keywords_any(vec![
            KeywordAbility::Deathtouch,
            KeywordAbility::Hexproof,
            KeywordAbility::Reach,
            KeywordAbility::Trample,
        ]),
        reveal: true,
    }]
}
