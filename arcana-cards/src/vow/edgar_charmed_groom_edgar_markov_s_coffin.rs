//! Edgar, Charmed Groom // Edgar Markov's Coffin — `{2}{W}{B}` Legendary Creature
//! — Vampire Noble 4/4.
//!
//! Front face — Edgar, Charmed Groom:
//!   Other Vampires you control get +1/+1.
//!   When Edgar dies, return it to the battlefield transformed under its owner's
//!     control.
//!
//! Back face — Edgar Markov's Coffin — Legendary Artifact:
//!   At the beginning of your upkeep, create a 1/1 white and black Vampire creature
//!     token with lifelink and put a bloodline counter on Edgar Markov's Coffin. Then
//!     if there are three or more bloodline counters on it, remove those counters and
//!     transform it.
//!
//! # GAP
//! - Front "Other Vampires you control get +1/+1" is modeled as a filtered pump
//!   installed once from an ungated ETB trigger with
//!   `Duration::WhileSourceShowsFace(0)` — live while Edgar (front) shows, dimmed
//!   while the Coffin shows. NOTE: no exclude-source filter builder, and Edgar is
//!   himself a Vampire, so he pumps himself +1/+1 too (accepted approximation of
//!   "other").
//! - Back upkeep "then if there are three or more bloodline counters on it, remove
//!   those counters and transform it" is a resolution-time conditional on the source's
//!   own counter total. No exposed Effect models "if source has N counters, then
//!   remove + transform" (Effect::Conditional's Condition surface lacks a
//!   counters-on-source predicate). The token creation + counter placement is modeled;
//!   the conditional removal/transform is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Edgar, Charmed Groom");
    let vampire = reg.interner_mut().intern("Vampire");
    let noble = reg.interner_mut().intern("Noble");
    // Pre-intern the token subtype + counter name for the back-face upkeep effect.
    let _vampire_token = reg.interner_mut().intern("Vampire");
    let _bloodline = reg.interner_mut().intern("bloodline");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(vampire);
    front_subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // "Other Vampires you control get +1/+1" — installed from the ETB
        // trigger with Duration::WhileSourceShowsFace(0) below.
        ..Default::default()
    };

    // Back face: Edgar Markov's Coffin — Legendary Artifact.
    let back_name = reg.interner_mut().intern("Edgar Markov's Coffin");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white() | ColorSet::black(),
            types: TypeLine::ARTIFACT.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: when Edgar dies, return it transformed.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies_return_transformed,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back: at the beginning of your upkeep, make a Vampire token + bloodline counter.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: coffin_upkeep,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // ETB (either face): install the front-face Vampire anthem once;
            // the face-gated duration dims it while the Coffin shows. Left
            // ungated so re-entry transformed still installs it.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_vampire_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 1 front-only, trigger 2 back-only.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn install_vampire_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Front face: "Other Vampires you control get +1/+1" — live while the
    // front face shows. NOTE: no exclude-source filter builder, and Edgar is
    // himself a Vampire, so he pumps himself +1/+1 (accepted approximation
    // of "other").
    let vampire = reg
        .interner()
        .lookup("Vampire")
        .expect("Vampire interned during register()");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            ObjectFilter::creature()
                .controlled_by(ControllerConstraint::You)
                .with_subtype_sym(vampire),
            1,
            1,
            Duration::WhileSourceShowsFace(0),
        ),
    }]
}

fn on_dies_return_transformed(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let dying = trig.dying_object().unwrap_or(trig.source);
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: dying },
        Effect::Transform { target: dying },
    ]
}

fn coffin_upkeep(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Create a 1/1 white and black Vampire token with lifelink.
    let vampire = reg
        .interner()
        .lookup("Vampire")
        .expect("Vampire interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    let token = TokenDefinition {
        name: vampire,
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        abilities: vec![],
    };
    let bloodline = reg
        .interner()
        .lookup("bloodline")
        .expect("bloodline interned during register()");
    // GAP: "Then if there are three or more bloodline counters on it, remove those
    // counters and transform it." — no exposed Effect for a source-counter-threshold
    // conditional remove + transform. Token creation + counter placement modeled below.
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Named(bloodline),
            count: 1,
        },
    ]
}
