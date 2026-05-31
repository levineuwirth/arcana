//! Unstable Glyphbridge // Sandswirl Wanderglyph (transforming DFC).
//!
//! Front face: Artifact {3}{W}{W}. "When this artifact enters, if you cast it,
//! for each player, choose a creature with power 2 or less that player controls.
//! Then destroy all creatures except creatures chosen this way." plus a Craft
//! ability that transforms it.
//!
//! Back face: Artifact Creature — Golem, 4/4, Flying, with two opponent-locking
//! abilities.
//!
//! GAP: the front ETB ("choose a creature per player with power 2 or less, then
//! destroy all creatures except the chosen ones") is a per-player chosen-exception
//! mass destruction — not expressible with the available effect surface (no
//! "destroy all except a chosen set" primitive). Emitting Vec::new() for it.
//! GAP: Craft (cost: exile this + another artifact, return transformed, sorcery
//! speed) is not modeled as an activated-ability cost shape — the transform side
//! is represented by the back face but the Craft activation cannot be authored.
//! GAP: back-face "whenever an opponent casts a spell during their turn, they
//! can't attack you" and "each opponent who attacked you this turn can't cast
//! spells" are conditional can't-attack / can't-cast locks with no effect surface.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unstable Glyphbridge");
    let golem = reg.interner_mut().intern("Golem");
    let back_name = reg.interner_mut().intern("Sandswirl Wanderglyph");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };

    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(golem);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front ETB — chosen-exception mass destruction; see GAP.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: front_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn front_etb(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if you cast it, for each player choose a creature with power 2 or
    // less they control, then destroy all creatures except those chosen" —
    // no chosen-exception mass-destruction primitive.
    Vec::new()
}
