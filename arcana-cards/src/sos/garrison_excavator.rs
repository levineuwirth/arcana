//! Garrison Excavator — `{3}{R}` 3/4 Creature — Orc Sorcerer with Menace.
//! "Whenever one or more cards leave your graveyard, create a 2/2 red and white
//! Spirit creature token."
//!
//! Menace is a base characteristic. The graveyard-leave trigger is approximated
//! as a graveyard -> battlefield zone change (the common way a card leaves your
//! graveyard), minting the Spirit (Desecrated Tomb pattern).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garrison Excavator");
    let orc = reg.interner_mut().intern("Orc");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    // Pre-intern the token subtype for resolve-time lookup.
    let _spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(sorcerer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "whenever one or more CARDS LEAVE your graveyard" — ZoneChange
            // requires a concrete `to` zone; modeled as graveyard -> battlefield
            // only (reanimation), missing exile/hand/library exits, and firing
            // per card rather than once per batch.
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::new().controlled_by(ControllerConstraint::You),
                from: Some(Zone::Graveyard(0)),
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: make_spirit,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_spirit(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spirit,
            colors: ColorSet::red() | ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
