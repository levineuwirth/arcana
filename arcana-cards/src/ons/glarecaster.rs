//! Glarecaster — `{4}{W}{W}` 3/3 Bird Cleric.
//! Flying.
//! {5}{W}: The next time damage would be dealt to this creature and/or
//! you this turn, that damage is dealt to any target instead.
//!
//! Flying is a base keyword. The activated ability redirects damage from
//! this creature AND its controller to a chosen target (two
//! RedirectDamage installs to "any target"). Fidelity gap: the redirect
//! is installed for the rest of the turn (ReplacementDuration::EndOfTurn)
//! rather than firing exactly the "next time" once.

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
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glarecaster");
    let bird = reg.interner_mut().intern("Bird");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{W}: The next time damage would be dealt to this creature and/or you this turn, that damage is dealt to any target instead.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: redirect_to_target,
            }),
    )
}

fn redirect_to_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let to = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![
        Effect::RedirectDamage {
            from: DamageTarget::Object(ctx.source),
            to,
            duration: ReplacementDuration::EndOfTurn,
        },
        Effect::RedirectDamage {
            from: DamageTarget::Player(ctx.controller),
            to,
            duration: ReplacementDuration::EndOfTurn,
        },
    ]
}
