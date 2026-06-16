//! Weeping Angel — `{1}{U}{B}` 2/2 Artifact Creature — Alien Angel.
//! Flash, first strike, vigilance.
//! "Whenever an opponent casts a creature spell, this creature isn't a
//! creature until end of turn." + a combat-damage replacement that
//! shuffles the damaged creature into its owner's library.
//!
//! The opponent-creature-cast trigger fires, but "isn't a creature
//! until end of turn" (removing the creature type from itself) has no
//! demonstrated primitive and is GAP'd. The combat-damage replacement
//! ("prevent that damage and shuffle it in") is a replacement effect
//! with no demonstrated primitive and is GAP'd entirely.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Weeping Angel");
    let alien = reg.interner_mut().intern("Alien");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Flash,
            KeywordAbility::FirstStrike,
            KeywordAbility::Vigilance,
        ],
        ..Default::default()
    };

    // GAP: combat-damage replacement ("prevent that damage and that
    // creature's owner shuffles it into their library") — no primitive.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(arcana_core::targets::ObjectFilter::creature()),
                caster: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: becomes_noncreature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn becomes_noncreature(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "this creature isn't a creature until end of turn" — removing
    // the creature type from itself has no demonstrated primitive.
    Vec::new()
}
