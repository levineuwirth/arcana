//! Spirit en-Kor — `{3}{W}` 2/2 Creature — Kor Spirit with Flying.
//!
//! * Flying.
//! * `{0}: The next 1 damage that would be dealt to this creature this turn is
//!   dealt to target creature you control instead.` (Modeled via
//!   RedirectDamage from this creature to the chosen creature; the engine's
//!   redirect is "the next damage" — the exact 1-damage cap is a fidelity gap.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spirit en-Kor");
    let kor = reg.interner_mut().intern("Kor");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "{0}: The next 1 damage that would be dealt to this creature this
            // turn is dealt to target creature you control instead."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{0}: The next 1 damage that would be dealt to this creature this turn is dealt to target creature you control instead.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{0}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: redirect_damage,
            }),
    )
}

fn redirect_damage(
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
    // GAP: "the next 1 damage" — RedirectDamage redirects the next damage event
    // in full; the exact 1-damage cap is not modeled.
    vec![Effect::RedirectDamage {
        from: DamageTarget::Object(ctx.source),
        to: DamageTarget::Object(*id),
        duration: ReplacementDuration::EndOfTurn,
    }]
}
