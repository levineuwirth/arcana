//! Venat, Heart of Hydaelyn // Hydaelyn, the Mothercrystal
//!
//! Front (Venat, Heart of Hydaelyn): Legendary Creature — Elder Wizard, {1}{W}{W}, 3/3.
//! Whenever you cast a legendary spell, draw a card. This ability triggers only once each turn.
//! Hero's Sundering — {7}{T}: Exile target nonland permanent. Transform Venat. Activate only as a sorcery.
//!
//! Back (Hydaelyn, the Mothercrystal): Legendary Creature — God.
//! Indestructible.
//! Blessing of Light — At the beginning of combat on your turn, put a +1/+1 counter on another
//! target creature you control. Until your next turn, it gains indestructible.
//! If that creature is legendary, draw a card.
//!
//! Indestructible keyword on the back face is in characteristics. "Blessing of
//! Light" is a back-face triggered ability, now wired (gated to face 1): at the
//! beginning of combat on your turn, put a +1/+1 counter on another target
//! creature you control; until your next turn it gains indestructible
//! (Effect::GrantKeyword, Duration::UntilYourNextTurn); and if that creature is
//! legendary, draw a card — the resolver reads the chosen target's supertypes
//! from `state` and conditionally appends the draw.
//! GAP: "another target creature you control" — self-exclusion is not expressible
//! on the target filter (documented fidelity gap shared by every "another target"
//! card); the filter restricts to creatures you control.
//! GAP: "Activate only as a sorcery" — speed restriction not separately enforced.
//! "This ability triggers only once each turn" — TriggerFrequency::OncePerTurn used
//! for the front legendary-cast draw.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationZone, CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Venat, Heart of Hydaelyn");

    let elder_sub = reg.interner_mut().intern("Elder");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Hydaelyn, the Mothercrystal");
    let god_sub = reg.interner_mut().intern("God");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(god_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            keywords: vec![KeywordAbility::Indestructible],
            ..Default::default()
        },
        spell_ability: None,
    };

    // Exile target nonland permanent filter
    let nonland_filter = ObjectFilter::new().without_types(TypeLine::LAND.into());

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Triggered ability: whenever you cast a legendary spell, draw a card (once per turn)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_legendary_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            })
            // Hero's Sundering activated ability: {7}{T}: exile target nonland permanent, transform
            .with_activated_ability(ActivatedAbilityDef {
                text: "Hero's Sundering — {7}, {T}: Exile target nonland permanent. Transform Venat. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{7}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(nonland_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: heros_sundering,
            })
            // Back face — Blessing of Light: at the beginning of combat on your
            // turn, put a +1/+1 counter on another target creature you control;
            // until your next turn it gains indestructible; if that creature is
            // legendary, draw a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: blessing_of_light,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // Front legendary-cast draw fires only on the front face; Blessing of
            // Light only on the back face.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn blessing_of_light(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let mut effects = vec![
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::UntilYourNextTurn(trig.controller),
        },
    ];
    // "If that creature is legendary, draw a card."
    let is_legendary = state
        .objects
        .get(*id)
        .map(|o| o.characteristics.supertypes.is_legendary())
        .unwrap_or(false);
    if is_legendary {
        effects.push(Effect::DrawCards {
            player: trig.controller,
            count: 1,
        });
    }
    effects
}

fn on_legendary_cast(
    _state: &arcana_core::state::GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}

fn heros_sundering(
    _state: &arcana_core::state::GameState,
    ctx: &arcana_core::registry::ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::Transform { target: ctx.source },
    ]
}
