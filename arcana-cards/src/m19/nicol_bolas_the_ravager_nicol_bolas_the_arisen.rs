//! Nicol Bolas, the Ravager // Nicol Bolas, the Arisen
//! Front: `{1}{U}{B}{R}` Legendary Creature — Elder Dragon 4/4. Flying.
//! When Nicol Bolas enters, each opponent discards a card.
//! `{4}{U}{B}{R}`: Exile Nicol Bolas, then return him to the battlefield
//! transformed. Activate only as a sorcery.
//! Back: Legendary Planeswalker — Bolas (starting loyalty 7).
//! +2: Draw two cards.
//! −3: Nicol Bolas deals 10 damage to target creature or planeswalker.
//! −4: Put target creature or planeswalker card from a graveyard onto the
//!   battlefield under your control.
//! −12: Exile all but the bottom card of target player's library.
//!
//! GAP: back-face loyalty abilities are authored with face_gate: Some(1) but
//!   face-gating on ActivatedAbilityDef is only checked if the engine consults
//!   it; currently ActivatedAbilityDef does not support face_gate — filed as
//!   engine debt. Abilities are present on the CardDefinition regardless of face.
//! GAP: −12 "exile all but the bottom card of target player's library" —
//!   no Effect variant for "exile all but one card of library". Emitting
//!   Vec::new().
//! GAP: transform activation "exile Nicol Bolas, then return him transformed"
//!   is modeled as a direct Effect::Transform (skips the exile-and-return step).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nicol Bolas, the Ravager");
    let elder_sub = reg.interner_mut().intern("Elder");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder_sub);
    subtypes.0.insert(dragon_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        keywords: vec![KeywordAbility::Flying],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // Back face: Nicol Bolas, the Arisen — Legendary Planeswalker — Bolas, loyalty 7
    let back_name = reg.interner_mut().intern("Nicol Bolas, the Arisen");
    let bolas_sub = reg.interner_mut().intern("Bolas");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(bolas_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
            types: TypeLine::PLANESWALKER.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            loyalty: Some(7),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face ETB: each opponent discards a card
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Front face activated: {4}{U}{B}{R} — transform (sorcery speed)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{U}{B}{R}: Exile Nicol Bolas, the Ravager, then return him to the battlefield transformed. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{U}{B}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: transform_self,
            })
            // Back face +2: Draw two cards
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Draw two cards.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1), // back face only
                effect: plus_two_draw,
            })
            // Back face −3: deal 10 damage to target creature or planeswalker
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Nicol Bolas deals 10 damage to target creature or planeswalker.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(
                            TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER),
                        ),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: minus_three_damage,
            })
            // Back face −4: reanimate target creature or planeswalker from a graveyard
            .with_activated_ability(ActivatedAbilityDef {
                text: "-4: Put target creature or planeswalker card from a graveyard onto the battlefield under your control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new().with_types_any(
                            TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER),
                        ),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: minus_four_reanimate,
            })
            // Back face −12: GAP
            .with_activated_ability(ActivatedAbilityDef {
                text: "-12: Exile all but the bottom card of target player's library.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 12)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: minus_twelve,
            }),
    )
}

fn etb_discard(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    arcana_core::script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::Discard {
            player: opp,
            count: 1,
            choice: arcana_core::effects::DiscardChoice::ControllerChooses,
        })
        .collect()
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should exile then return transformed; modeled as direct transform.
    vec![Effect::Transform { target: ctx.source }]
}

fn plus_two_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 2 }]
}

fn minus_three_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 10,
    }]
}

fn minus_four_reanimate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}

fn minus_twelve(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile all but the bottom card of target player's library" —
    // no Effect variant for mass-library-exile. Emitting Vec::new().
    Vec::new()
}
