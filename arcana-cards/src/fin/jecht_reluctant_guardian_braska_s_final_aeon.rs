//! Jecht, Reluctant Guardian // Braska's Final Aeon
//!
//! Front face: {3}{B} Legendary Creature — Human Warrior 4/3
//! Menace
//! Whenever Jecht deals combat damage to a player, you may exile it, then return it to the
//! battlefield transformed under its owner's control.
//!
//! Back face: Legendary Enchantment Creature — Saga Nightmare
//! (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
//! I, II — Jecht Beam — Each opponent discards a card and you draw a card.
//! III — Ultimate Jecht Shot — Each opponent sacrifices two creatures of their choice.
//! Menace
//!
//! GAP: Back-face-only Saga chapter mechanics not modeled (abilities live on CardDefinition, not
//!      the face; Saga chapter triggers on back face would need a separate Saga engine path).
//! GAP: "Each opponent sacrifices two creatures of their choice" — opponent-choice sacrifice
//!      not expressible (Sacrifice effect uses controller-chooses only; whole chapter III GAP'd).
//! GAP: Transform trigger condition is "whenever Jecht deals combat damage to a player" —
//!      approximated via TriggerCondition::DamageDealt with combat_only: true.
//! GAP: "you may exile it, then return it transformed" — exile+return-transformed not a
//!      single Effect variant; approximated as Effect::Transform.
//! GAP: back face P/T not given in card spec (Saga+Creature hybrid — left unset).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jecht, Reluctant Guardian");
    let human_sub = reg.interner_mut().intern("Human");
    let warrior_sub = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(warrior_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        keywords: vec![KeywordAbility::Menace],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Back face: Braska's Final Aeon — Legendary Enchantment Creature — Saga Nightmare
    let back_name = reg.interner_mut().intern("Braska's Final Aeon");
    let saga_sub = reg.interner_mut().intern("Saga");
    let nightmare_sub = reg.interner_mut().intern("Nightmare");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(saga_sub);
    back_subtypes.0.insert(nightmare_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            keywords: vec![KeywordAbility::Menace],
            // GAP: back face is a Saga+Creature hybrid; P/T not given in card spec.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: whenever Jecht deals combat damage to a player, transform.
            // Approximated as DamageDealt with combat_only: true targeting a player.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: jecht_damage_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // GAP: back-face-only Saga chapter triggered abilities not modeled.
    )
}

fn jecht_damage_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may exile it, then return it to the battlefield transformed."
    // GAP: exile+return-transformed not a single Effect; using Transform as approximation.
    // GAP: "you may" — optional choice not modeled; fires unconditionally.
    vec![Effect::Transform { target: trig.source }]
}
