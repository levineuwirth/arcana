//! Vorinclex // The Grand Evolution — `{3}{G}{G}` Legendary Creature — Phyrexian Praetor 6/6.
//! Trample, reach. ETB: search library for up to two Forest cards, reveal them, put into hand.
//! {6}{G}{G}: Exile Vorinclex, return transformed (sorcery speed).
//! Back face: The Grand Evolution — Enchantment — Saga.
//! I — Mill ten, put up to two creature cards from among milled onto battlefield.
//! II — Distribute seven +1/+1 counters among any number of target creatures you control.
//! III — Until end of turn, creatures you control gain "{1}: This creature fights target creature you don't control."
//!        Exile this Saga, return it front face up.
//!
//! GAP: Saga back face chapter abilities (I, II, III) not auto-installed on transform — back-face-only triggered abilities not modeled.
//! GAP: "up to two Forest cards" — TutorToHand is single-pick; modeled as one Forest.
//! GAP: "Distribute seven +1/+1 counters among any number of targets" — multi-target distribution not expressible.
//! GAP: Chapter III "{1}: fight" activated ability grant is not expressible.
//! GAP: {6}{G}{G} sorcery-speed activated exile+transform — not expressible with ActivationCost.
//! GAP: "put up to two creature cards from among the milled cards onto the battlefield" — not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vorinclex");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let praetor_sub = reg.interner_mut().intern("Praetor");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian_sub);
    subtypes.0.insert(praetor_sub);

    // Pre-intern Forest for use in trigger handler lookup
    let _forest_sub = reg.interner_mut().intern("Forest");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Reach],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("The Grand Evolution");
    let saga_sub = reg.interner_mut().intern("Saga");

    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(saga_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: search library for Forest card, put into hand
            // GAP: "up to two Forest cards" — TutorToHand is single; modeled as one.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_search_forest,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: {6}{G}{G} sorcery-speed activated exile+return-transformed — not expressible.
        // GAP: back-face-only Saga chapter triggers not auto-installed on transform.
    )
}

fn etb_search_forest(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "up to two Forest cards" — modeled as single TutorToHand for a Forest land.
    let forest_name = reg.interner().lookup("Forest");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter {
            name: forest_name,
            ..ObjectFilter::default()
        },
        reveal: true,
    }]
}
