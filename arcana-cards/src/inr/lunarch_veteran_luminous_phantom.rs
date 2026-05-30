//! Lunarch Veteran // Luminous Phantom
//! Front: `{W}` Creature — Human Cleric 1/1.
//! Whenever another creature you control enters, you gain 1 life.
//! Disturb {1}{W} (You may cast this card from your graveyard transformed for its disturb cost.)
//!
//! Back: "Luminous Phantom" — Creature — Spirit Cleric with Flying.
//! Whenever another creature you control leaves the battlefield, you gain 1 life.
//! If Luminous Phantom would be put into a graveyard from anywhere, exile it instead.
//!
//! # GAPs
//! - Disturb keyword (alternative cast from graveyard transformed): not in the engine's
//!   keyword surface; not modeled.
//! - "If Luminous Phantom would be put into a graveyard from anywhere, exile it instead":
//!   replacement effect; not expressible in current engine.
//! - Back-face triggered ability (creature leaves battlefield → gain 1 life) not
//!   auto-installed on transform.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lunarch Veteran");
    let human_sub = reg.interner_mut().intern("Human");
    let cleric_sub = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(cleric_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Luminous Phantom — Creature — Spirit Cleric with Flying
    let back_name = reg.interner_mut().intern("Luminous Phantom");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let cleric_sub2 = reg.interner_mut().intern("Cleric");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit_sub);
    back_subtypes.0.insert(cleric_sub2);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            // GAP: "If Luminous Phantom would be put into a graveyard, exile it instead"
            // — replacement effect not expressible.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: "Whenever another creature you control enters, you gain 1 life."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(), // "another creature" — exclude self; use nontoken as proxy
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: gain_one_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: back-face triggered ability (creature leaves → gain 1 life) not auto-installed.
        // GAP: Disturb keyword not modeled (not in engine keyword surface).
    )
}

fn gain_one_life(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 1,
    }]
}
