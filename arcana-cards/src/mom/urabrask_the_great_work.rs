//! Urabrask // The Great Work — `{2}{R}{R}` Legendary Creature — Phyrexian Praetor 4/4.
//!
//! Front face — Urabrask:
//!   First strike.
//!   Whenever you cast an instant or sorcery spell, Urabrask deals 1 damage to
//!     target opponent. Add {R}.
//!   {R}: Exile Urabrask, then return it to the battlefield transformed under its
//!     owner's control. Activate only as a sorcery and only if you've cast three
//!     or more instant and/or sorcery spells this turn.
//!
//! Back face — The Great Work — Enchantment — Saga:
//!   I — This Saga deals 3 damage to target opponent and each creature they control.
//!   II — Create three Treasure tokens.
//!   III — Until end of turn, you may cast instant and sorcery spells from any
//!         graveyard. Exile this Saga, then return it (front face up).
//!
//! # GAP
//! - Back-face Saga chapter abilities (I/II/III) are not auto-installed on transform;
//!   the Saga back is declared but its chapter triggers are engine debt (same posture
//!   as Sheoldred // The True Scriptures).
//! - Activated ability's "only if you've cast three or more instant/sorcery spells
//!   this turn" precondition is not expressible as an ActivationCost gate; modeled as
//!   always-available (GAP on the condition).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urabrask");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let praetor = reg.interner_mut().intern("Praetor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(praetor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // Back face: The Great Work — Enchantment — Saga.
    let back_name = reg.interner_mut().intern("The Great Work");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(saga_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Whenever you cast an instant or sorcery spell: deal 1 to target opponent, add {R}.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_cast_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            })
            // {R}: Exile Urabrask, then return it transformed.
            // GAP: "only as a sorcery and only if you've cast 3+ instant/sorcery
            // spells this turn" precondition not expressible as an ActivationCost gate.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}: Exile Urabrask, then return it to the battlefield transformed.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: activate_exile_transform,
            })
            // Trigger 1 is front-only.
            .with_trigger_face_gate(1, 0),
    )
}

fn on_cast_spell(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return vec![Effect::AddMana {
            player: trig.controller,
            mana: vec![ManaUnit::plain(ManaColor::Red, trig.source)],
        }];
    };
    let TargetChoice::Player(p) = target else {
        return vec![Effect::AddMana {
            player: trig.controller,
            mana: vec![ManaUnit::plain(ManaColor::Red, trig.source)],
        }];
    };
    vec![
        Effect::DealDamage {
            target: DamageTarget::Player(*p),
            amount: 1,
            source: trig.source,
        },
        Effect::AddMana {
            player: trig.controller,
            mana: vec![ManaUnit::plain(ManaColor::Red, trig.source)],
        },
    ]
}

fn activate_exile_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::ExilePermanent { target: ctx.source },
        Effect::ReturnFromExileToBattlefield { target: ctx.source },
        Effect::Transform { target: ctx.source },
    ]
}
