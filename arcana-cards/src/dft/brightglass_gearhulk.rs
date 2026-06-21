//! Brightglass Gearhulk — `{G}{G}{W}{W}` 4/4 Artifact Creature — Construct.
//! First strike, trample.
//! When this creature enters, you may search your library for up to two artifact,
//! creature, and/or enchantment cards with mana value 1 or less, reveal them, put
//! them into your hand, then shuffle.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brightglass Gearhulk");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_tutor_two,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_tutor_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Fidelity note: "up to two ... you may" is modeled as two non-optional tutors
    // for an artifact/creature/enchantment card with mana value 1 or less.
    let filter = arcana_core::targets::ObjectFilter::new()
        .with_types_any(TypeLine(
            TypeLine::ARTIFACT | TypeLine::CREATURE | TypeLine::ENCHANTMENT,
        ))
        .with_max_cmc(1);
    vec![
        Effect::TutorToHand {
            player: trig.controller,
            filter: filter.clone(),
            reveal: true,
        },
        Effect::TutorToHand {
            player: trig.controller,
            filter,
            reveal: true,
        },
    ]
}
