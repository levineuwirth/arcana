//! Whisperwood Elemental — `{3}{G}{G}` 4/4 Elemental.
//! At the beginning of your end step, manifest the top card of your library.
//! Sacrifice this creature: Until end of turn, face-up nontoken creatures
//! you control gain "When this creature dies, manifest the top card of your
//! library."
//!
//! Ability 1: end-step trigger → Effect::Manifest.
//! Ability 2: sacrifice-self activation grants each face-up nontoken
//! creature you control a death-trigger (manifest) until end of turn, built
//! one GrantTriggeredAbility per matching id inside a Sequence (ForEach does
//! NOT substitute per-id). "Manifest" the Scryfall keyword is not an
//! emittable KeywordAbility — it is realized as Effect::Manifest.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Whisperwood Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_manifest,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice this creature: Until end of turn, face-up nontoken creatures you control gain \"When this creature dies, manifest the top card of your library.\"".into(),
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_death_manifest,
            }),
    )
}

fn end_step_manifest(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Manifest { player: trig.controller }]
}

fn grant_death_manifest(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .nontoken(),
        ctx.controller,
    );
    let grants: Vec<Effect> = ids
        .into_iter()
        .map(|id| Effect::GrantTriggeredAbility {
            target: id,
            ability: Box::new(TriggeredAbilityDef {
                id: GRANTED_TRIGGER_ID_BASE + 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: granted_dies_manifest,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
            duration: Duration::EndOfTurn,
        })
        .collect();
    vec![Effect::Sequence(grants)]
}

fn granted_dies_manifest(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Manifest { player: trig.controller }]
}
