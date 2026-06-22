//! Hostility — `{3}{R}{R}{R}` 6/6 Elemental Incarnation.
//!
//! * Haste.
//! * If a spell you control would deal damage to an opponent, prevent
//!   that damage. Create a 3/1 red Elemental Shaman creature token with
//!   haste for each 1 damage prevented this way. (GAP — static damage
//!   replacement.)
//! * When Hostility is put into a graveyard from anywhere, shuffle it
//!   into its owner's library.

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
    let name = reg.interner_mut().intern("Hostility");
    let elemental = reg.interner_mut().intern("Elemental");
    let incarnation = reg.interner_mut().intern("Incarnation");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(incarnation);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "If a spell you control would deal damage to an opponent,
    // prevent that damage. Create a 3/1 red Elemental Shaman with haste
    // for each 1 damage prevented this way." This is a static
    // damage-replacement effect that mints tokens proportional to the
    // prevented damage — not expressible as a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars)
            // "When Hostility is put into a graveyard from anywhere,
            // shuffle it into its owner's library." SelfDies is the
            // closest condition (battlefield→graveyard); the "from
            // anywhere" breadth is a GAP.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: shuffle_into_library,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn shuffle_into_library(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Put the card from the graveyard into the library, then shuffle.
    vec![
        Effect::PutOnTopOfLibrary { target: trig.source },
        Effect::Shuffle { player: trig.controller },
    ]
}
