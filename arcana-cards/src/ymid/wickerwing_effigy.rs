//! Wickerwing Effigy — `{3}` 1/4 colorless Artifact Creature — Scarecrow with
//! Defender.
//!
//! Oracle text:
//! * "Defender"
//! * "You may look at the top card of your library any time." — pure static
//!   information permission. // GAP: static — top-card visibility not modeled.
//! * "You may cast creature spells from the top of your library." — pure
//!   static play-permission. // GAP: static — cast-from-top permission not
//!   modeled.
//! * "Whenever you cast a creature spell from your library, it becomes a black
//!   Bird in addition to its other colors and types, has flying, and has base
//!   power and toughness 1/1." — SpellCast trigger filtered to a creature
//!   spell you cast. The effect modifies the just-cast spell object, but there
//!   is no accessor exposing that stack object's id, and SpellCast cannot be
//!   restricted to "from your library". // GAP: effect — cannot identify the
//!   cast spell object to apply color/type/keyword/base-P-T changes.

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
    let name = reg.interner_mut().intern("Wickerwing Effigy");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: bird_the_cast_spell,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn bird_the_cast_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: effect — no accessor exposes the just-cast spell object id, and
    // SpellCast can't be restricted to "from your library"; cannot apply the
    // black-Bird / flying / base-1/1 modifications to the correct object.
    Vec::new()
}
