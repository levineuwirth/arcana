//! Poppet Stitcher // Poppet Factory — `{2}{U}` Creature — Human Wizard 2/3.
//!
//! Front face (Poppet Stitcher):
//! - Whenever you cast an instant or sorcery spell, create a 2/2 black Zombie
//!   creature token with decayed.
//! - At the beginning of your upkeep, if you control three or more creature
//!   tokens, you may transform this creature.
//!
//! Back face (Poppet Factory): Artifact (no P/T).
//! - Creature tokens you control lose all abilities and have base power and
//!   toughness 3/3.
//! - At the beginning of your upkeep, you may transform this artifact.
//!
//! # GAPs
//! - "Decayed" keyword on Zombie token: Decayed is not in the engine keyword
//!   list. The token is created without the Decayed keyword.
//! - Upkeep transform condition "if you control three or more creature tokens"
//!   — intervening-if conditions are not structured in the API as a script
//!   expression; emitting without the count-gate.
//! - Back face "creature tokens lose all abilities and have base 3/3" — this
//!   is a back-face-only continuous static ability; not modelable (GAP:
//!   back-face-only triggered/static ability not modeled).
//! - Back face upkeep transform — back-face-only triggered ability not modeled.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Poppet Stitcher");
    let human_sub = reg.interner_mut().intern("Human");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let zombie_sub = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let _ = zombie_sub; // interned for resolver lookup

    // Back face: Poppet Factory — Artifact (non-creature)
    let back_name = reg.interner_mut().intern("Poppet Factory");
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::blue(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    let back = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    // Trigger 1: When you cast an instant or sorcery spell, create a 2/2
    // black Zombie creature token.
    let instant_or_sorcery_filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY))
        .controlled_by(ControllerConstraint::You);

    // Trigger 2: At the beginning of your upkeep, if you control 3+ creature
    // tokens, you may transform. GAP: count-gate not expressible as
    // intervening_if.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(instant_or_sorcery_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: create_zombie_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: maybe_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: back-face-only triggered ability (upkeep transform back) not
        //      modeled
        // GAP: back-face-only static ability (tokens base 3/3, lose abilities)
        //      not modeled
    )
}

fn create_zombie_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie")
        .expect("Zombie interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(zombie);
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        // GAP: "decayed" keyword not in engine keyword list — token omits it
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}

fn maybe_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you control three or more creature tokens" condition not
    // enforced — transform fires unconditionally at upkeep.
    vec![Effect::Transform { target: trig.source }]
}
