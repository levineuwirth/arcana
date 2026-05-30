//! Umara Wizard // Umara Skyfalls — MDFC
//!
//! Front: {4}{U} Creature — Merfolk Wizard 4/3
//! Whenever you cast an instant, sorcery, or Wizard spell, this creature
//! gains flying until end of turn.
//! Wired as two separate SpellCast triggers:
//!   Trigger 1: instant or sorcery cast by you → grant flying.
//!   Trigger 2: Wizard spell cast by you → grant flying.
//! (Two triggers may fire on a Wizard instant/sorcery — both grant flying,
//! which is harmless since it's the same duration effect.)
//!
//! Back: Umara Skyfalls — Land
//! This land enters tapped.
//! {T}: Add {U}.
//! GAP: "enters tapped" behavior and the {T}: Add {U} mana ability on the back
//! face are not modeled in the current engine (land MDFC back-face activations
//! are engine debt).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Umara Wizard");
    let merfolk_sub = reg.interner_mut().intern("Merfolk");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Back face: Umara Skyfalls — Land
    let back_name = reg.interner_mut().intern("Umara Skyfalls");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::LAND.into(),
            // GAP: "enters tapped" and "{T}: Add {U}" not modeled for land MDFC back.
            ..Default::default()
        },
        spell_ability: None,
    };

    // Build a Wizard subtype filter for trigger 2
    let wizard_filter = ObjectFilter::new()
        .with_subtypes_any(vec![wizard_sub]);

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            // Trigger 1: whenever you cast an instant or sorcery spell
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY))),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: grant_flying,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 2: whenever you cast a Wizard spell
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(wizard_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: grant_flying,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn grant_flying(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::Flying,
        duration: Duration::EndOfTurn,
    }]
}
