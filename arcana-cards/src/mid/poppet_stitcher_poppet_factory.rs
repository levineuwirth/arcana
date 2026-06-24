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
//! - Back face "creature tokens you control lose all abilities and have base
//!   power and toughness 3/3" — a FILTERED continuous static (over all creature
//!   tokens you control). The engine has only single-target `Effect::LoseAllAbilities`
//!   / `Effect::SetBasePT`; there is no `filtered_lose_abilities` / `filtered_set_pt`
//!   ContinuousEffect constructor to apply lose-abilities + set-base-P/T to a filter.
//!   Left GAP'd.
//!
//! The front upkeep transform is now count-gated ("if you control three or more
//! creature tokens") via an intervening-if, and the back-face "you may transform
//! this artifact" upkeep trigger is wired (face-gated).

use arcana_core::conditions;
use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine,
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
    // tokens, you may transform — count-gated via the if_three_creature_tokens
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
                // "if you control three or more creature tokens" — count-gate.
                intervening_if: Some(if_three_creature_tokens),
                effect: maybe_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face (Poppet Factory): "At the beginning of your upkeep, you may
            // transform this artifact." (Transforms back to the Poppet Stitcher front.)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: maybe_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Token-maker + front upkeep transform are front-only; the back upkeep
            // transform is back-only.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 0)
            .with_trigger_face_gate(3, 1),
        // GAP: back-face static "creature tokens you control lose all abilities and
        //      have base 3/3" — no filtered lose-abilities / set-base-P/T constructor.
    )
}

/// "if you control three or more creature tokens".
fn if_three_creature_tokens(
    state: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    let filter = ObjectFilter::creature().tokens_only();
    conditions::you_control_at_least(state, you, &filter, 3)
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
    // The front-face count-gate ("3+ creature tokens") is enforced as an
    // intervening_if on trigger 2; the back-face upkeep transform (trigger 3) is
    // unconditional. Both resolve to a self-transform.
    vec![Effect::Transform { target: trig.source }]
}
