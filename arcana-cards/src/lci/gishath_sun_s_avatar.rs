//! Gishath, Sun's Avatar — `{5}{R}{G}{W}` 7/6 Legendary Dinosaur Avatar with
//! Vigilance, Trample, and Haste.
//! Whenever Gishath deals combat damage to a player, reveal that many cards
//! from the top of your library. Put any number of Dinosaur creature cards
//! from among them onto the battlefield and the rest on the bottom of your
//! library in a random order.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gishath, Sun's Avatar");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: reveal_dinos,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn reveal_dinos(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal that-many cards, put any number of Dinosaur creature cards
    // from among them onto the battlefield, rest on bottom in random order" —
    // there is no Effect that reveals the top N (= combat damage) and
    // selectively puts matching cards onto the battlefield. RevealUntil takes
    // the FIRST match; DigTopN puts to HAND, not battlefield, and only one.
    Vec::new()
}
