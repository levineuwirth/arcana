//! Shaman en-Kor — `{1}{W}` 1/2 Kor Cleric Shaman.
//! "{0}: The next 1 damage that would be dealt to this creature this
//!   turn is dealt to target creature you control instead."
//! "{1}{W}: The next time a source of your choice would deal damage to
//!   target creature this turn, that damage is dealt to this creature
//!   instead."
//!
//! Both abilities are damage redirections; modeled with
//! `Effect::RedirectDamage` (CR 614.9 — the next damage is redirected).
//! The exact "1 damage" / "source of your choice" granularity is a
//! fidelity gap, but the redirection itself is faithful.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shaman en-Kor");
    let kor = reg.interner_mut().intern("Kor");
    let cleric = reg.interner_mut().intern("Cleric");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(cleric);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // {0}: next 1 damage to this creature → target creature you control.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{0}: The next 1 damage that would be dealt to this creature this turn is dealt to target creature you control instead.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: redirect_from_self,
            })
            // {1}{W}: next damage to target creature → this creature instead.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{W}: The next time a source of your choice would deal damage to target creature this turn, that damage is dealt to this creature instead.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: redirect_to_self,
            }),
    )
}

fn redirect_from_self(
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
    vec![Effect::RedirectDamage {
        from: DamageTarget::Object(ctx.source),
        to: DamageTarget::Object(*id),
        duration: ReplacementDuration::EndOfTurn,
    }]
}

fn redirect_to_self(
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
    vec![Effect::RedirectDamage {
        from: DamageTarget::Object(*id),
        to: DamageTarget::Object(ctx.source),
        duration: ReplacementDuration::EndOfTurn,
    }]
}
