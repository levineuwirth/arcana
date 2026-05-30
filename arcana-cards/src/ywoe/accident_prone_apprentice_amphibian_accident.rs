//! Accident-Prone Apprentice // Amphibian Accident
//!
//! Creature face: `{1}{R}` Otter Wizard 1/1.
//! "Whenever you cast a noncreature spell, Accident-Prone Apprentice perpetually gets +1/+1.
//!  This ability also triggers if Accident-Prone Apprentice is in exile."
//!
//! Adventure face: "Amphibian Accident" `{1}{U}` instant.
//! "Until end of turn, target creature loses all abilities and becomes a blue Frog
//!  with base power and toughness 1/1."
//!
//! # GAPs
//! - "Perpetually gets +1/+1" is an Alchemy/Arena mechanic (persistent cross-zone counter);
//!   the engine has no `Effect::Perpetually` variant. The trigger fires but produces `Vec::new()`.
//! - "This ability also triggers if ... in exile" — exile-zone trigger not supported by
//!   `trigger_zones`; only `Battlefield` is wired.
//! - Adventure resolve: "loses all abilities" is not a catalog Effect. `SetBasePT` models
//!   the 1/1 floor only. The color-change (becomes blue) and subtype-change (Frog) are GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Accident-Prone Apprentice");
    let otter_sub = reg.interner_mut().intern("Otter");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(otter_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Adventure face: "Amphibian Accident" {1}{U} instant
    let adv_name = reg.interner_mut().intern("Amphibian Accident");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid adv cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Until end of turn, target creature loses all abilities and becomes a blue Frog with base power and toughness 1/1.".into(),
        target_requirements: vec![TargetRequirement::target_creature()],
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure)
            // Triggered ability: whenever you cast a noncreature spell, perpetually gets +1/+1.
            // GAP: trigger_zones: only Battlefield; exile-zone trigger not supported.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().without_types(TypeLine::CREATURE.into()),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: noncreature_cast_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn noncreature_cast_trigger(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Perpetually gets +1/+1" is an Alchemy/Arena mechanic (persistent cross-zone stat
    // buff); no Effect::Perpetually variant exists. Returning empty until engine supports it.
    Vec::new()
}

fn adv_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "loses all abilities" is not a catalog Effect.
    // GAP: "becomes a blue Frog" — color-change and subtype-change not expressible.
    // Partial: set base P/T to 1/1 until end of turn.
    vec![Effect::SetBasePT {
        target: *id,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
    }]
}
