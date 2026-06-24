//! Docent of Perfection // Final Iteration — `{3}{U}{U}` Insect Horror 5/4 with Flying.
//!
//! Front face (Docent of Perfection):
//! - Flying
//! - Whenever you cast an instant or sorcery spell, create a 1/1 blue Human Wizard creature token.
//!   Then if you control three or more Wizards, transform this creature.
//!
//! Back face (Final Iteration): Eldrazi Insect, Flying, 5/4.
//! - GAP: "Wizards you control get +2/+1 and have flying" is a static continuous
//!   anthem (filtered pump + filtered keyword grant). It could be installed via
//!   ContinuousEffect on transform, but is left GAP'd here pending the static-on-
//!   transform idiom; the token-making trigger below is the wired part.
//! - Whenever you cast an instant or sorcery spell, create a 1/1 blue Human Wizard
//!   creature token. Wired as a back-face SpellCast trigger (gated to face 1) that
//!   creates the token WITHOUT the front face's "transform if 3+ Wizards" rider.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Docent of Perfection");
    let insect_sub = reg.interner_mut().intern("Insect");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect_sub);
    subtypes.0.insert(horror_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Flying],
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // Pre-intern token subtypes
    let _human_tok = reg.interner_mut().intern("Human");
    let _wizard_tok = reg.interner_mut().intern("Wizard");

    // Back face: Final Iteration — Eldrazi Insect, Flying
    let back_name = reg.interner_mut().intern("Final Iteration");
    let eldrazi_sub = reg.interner_mut().intern("Eldrazi");
    let back_insect_sub = reg.interner_mut().intern("Insect");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(eldrazi_sub);
    back_subtypes.0.insert(back_insect_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Flying],
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Front-face trigger: whenever YOU cast an instant or sorcery, create a 1/1 blue Human Wizard,
    // then if you control 3+ Wizards, transform.
    let instant_sorcery_filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: create a Wizard token, then transform if 3+ Wizards.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(instant_sorcery_filter.clone()),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cast_instant_or_sorcery,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: just create a Wizard token (no transform rider).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(instant_sorcery_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: back_cast_instant_or_sorcery,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

/// Build the 1/1 blue Human Wizard creature token shared by both faces.
fn wizard_token(reg: &CardRegistry) -> TokenDefinition {
    let wizard_id = reg.interner().lookup("Wizard");
    let human_id = reg.interner().lookup("Human");
    let mut wizard_subtypes = SubtypeSet::default();
    if let Some(h) = human_id {
        wizard_subtypes.0.insert(h);
    }
    if let Some(w) = wizard_id {
        wizard_subtypes.0.insert(w);
    }
    TokenDefinition {
        name: wizard_id.unwrap_or(0),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: wizard_subtypes,
        keywords: vec![],
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        abilities: vec![],
    }
}

fn back_cast_instant_or_sorcery(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: wizard_token(reg),
    }]
}

fn cast_instant_or_sorcery(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let create_token = Effect::CreateToken {
        controller: trig.controller,
        token: wizard_token(reg),
    };

    // Count wizards (including this creature itself if it's a Wizard — it's not, but we count
    // existing wizard tokens). If count >= 2, the new token will bring it to 3+.
    let wizard_filter = script::subtype_filter(reg, "Wizard")
        .controlled_by(ControllerConstraint::You);
    let wizard_count = script::count_matching(state, &wizard_filter, trig.controller);

    // If current wizard count >= 2, the new token will make it >= 3 → transform.
    if wizard_count >= 2 {
        vec![
            create_token,
            Effect::Transform { target: trig.source },
        ]
    } else {
        vec![create_token]
    }
}
