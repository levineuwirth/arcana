//! Jade Seedstones // Jadeheart Attendant ({3}{G}, green Artifact // Artifact Creature — Golem).
//!
//! Front (Artifact):
//!   When this artifact enters, distribute three +1/+1 counters among one, two, or three target
//!   creatures you control.
//!   Craft with creature {5}{G}{G} ({5}{G}{G}, Exile this artifact, Exile a creature you control or
//!   a creature card from your graveyard: Return this card transformed under its owner's control.
//!   Craft only as a sorcery.)
//! Back (Artifact Creature — Golem):
//!   When this creature enters, you gain life equal to the mana value of the exiled card used to
//!   craft it.
//!
//! GAPs:
//! - Front ETB "distribute three +1/+1 counters among one, two, or three target creatures you
//!   control": there is no distribute-counters effect (counters can be added to a single target,
//!   but a variable 1–3 target split of a fixed three-counter pool is not expressible). Emitted as
//!   an empty effect.
//! - Craft (the {5}{G}{G}, exile-this + exile-a-creature activated ability that returns the card
//!   transformed) is not modeled — there is no Craft activated-cost primitive. No activated ability
//!   is authored; the back face is declared via with_transform_back so the catalog records it.
//! - Back ETB "gain life equal to the mana value of the exiled card used to craft it" depends on
//!   the Craft exile that is not modeled, so the gained amount is undeterminable here. Emitted as an
//!   empty effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jade Seedstones");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };

    // Back face: Jadeheart Attendant — Artifact Creature — Golem.
    let back_name = reg.interner_mut().intern("Jadeheart Attendant");
    let golem = reg.interner_mut().intern("Golem");
    let mut back_subs = SubtypeSet::default();
    back_subs.0.insert(golem);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subs,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front ETB: distribute three +1/+1 counters (GAP — see module docs). Front-only.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: front_etb_distribute,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            // Back ETB: gain life equal to exiled card's mana value (GAP). Back-only.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: back_etb_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(2, 1),
    )
}

fn front_etb_distribute(_: &GameState, _trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: distribute three +1/+1 counters among 1–3 target creatures you control. No
    // distribute-counters effect exists (variable target split of a fixed counter pool).
    Vec::new()
}

fn back_etb_gain_life(_: &GameState, _trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: gain life equal to the mana value of the card exiled to Craft this. Craft is not
    // modeled, so the exiled card (and its mana value) are not tracked.
    Vec::new()
}
