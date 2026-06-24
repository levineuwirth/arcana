//! Henrika Domnathi // Henrika, Infernal Seer — `{2}{B}{B}` Legendary Vampire 1/3.
//! Front face: Flying. At the beginning of combat on your turn, choose one that
//! hasn't been chosen — each player sacrifices a creature; draw a card and lose 1
//! life; or transform Henrika.
//! Back face: Legendary Vampire with Flying, Deathtouch, Lifelink.
//! {1}{B}{B}: Each creature you control with flying, deathtouch, and/or lifelink
//! gets +1/+0 until end of turn.
//!
//! The back-face activated pump ({1}{B}{B}: each creature you control with
//! flying, deathtouch, and/or lifelink gets +1/+0 until end of turn) is wired
//! on the shared CardDefinition, gated to the back face (face_gate: Some(1)),
//! via a filtered-pump continuous effect.
//!
//! GAP: "choose one that hasn't been chosen" modal tracking not modeled; wired as
//! three separate triggered abilities on the front face (id 2/3/4) with no
//! exhaustion tracking. The engine will allow re-choosing the same mode.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Henrika Domnathi");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Henrika, Infernal Seer");
    let back_vampire_sub = reg.interner_mut().intern("Vampire");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_vampire_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![
                KeywordAbility::Flying,
                KeywordAbility::Deathtouch,
                KeywordAbility::Lifelink,
            ],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Trigger id=1: beginning of combat on your turn — fires the modal choice
            // GAP: true "choose one that hasn't been chosen" tracking not modeled;
            // three separate triggered abilities stand in for the modal choices.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: trig_sacrifice_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: trig_draw_lose_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: trig_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: "{1}{B}{B}: Each creature you control with flying,
            // deathtouch, and/or lifelink gets +1/+0 until end of turn."
            // Filtered temporary pump-all, gated to the back face.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}{B}: Each creature you control with flying, deathtouch, and/or lifelink gets +1/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1), // back face only
                effect: back_pump_keyworded_creatures,
            }),
    )
}

fn back_pump_keyworded_creatures(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Each creature you control with flying, deathtouch, and/or lifelink
    // gets +1/+0 until end of turn." — filtered pump over your matching
    // creatures, source-bound so cleanup is correct if Henrika leaves.
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_keywords_any(vec![
            KeywordAbility::Flying,
            KeywordAbility::Deathtouch,
            KeywordAbility::Lifelink,
        ]);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(ctx.source, filter, 1, 0, Duration::EndOfTurn),
    }]
}

fn trig_sacrifice_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each player" — only controller's sacrifice modeled; opponent sacrifice not modeled
    // as we lack "for each player" iteration in triggered context without script::all_players
    // which takes a state reference. We model controller's sacrifice as a representative effect.
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        count: 1,
    }]
}

fn trig_draw_lose_life(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::LoseLife {
            player: trig.controller,
            amount: 1,
        },
    ]
}

fn trig_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform {
        target: trig.source,
    }]
}
