//! Teferi, Time Raveler — `{1}{W}{U}` Legendary Planeswalker — Teferi,
//! starting loyalty 4.
//!
//! Each opponent can cast spells only any time they could cast a sorcery.
//! +1: Until your next turn, you may cast sorcery spells as though they had
//!     flash.
//! −3: Return up to one target artifact, creature, or enchantment to its
//!     owner's hand. Draw a card.
//!
//! The static is `CastTimingRestriction::OpponentsSorceryTiming`, read from
//! the battlefield by every cast path, Spell Queller's free cast included.
//! The +1 grants its controller a flash permission for sorceries until their
//! next turn, which outlasts Teferi (A1.5aa: both were GAP'd).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, CastTimingRestriction,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teferi, Time Raveler");
    let teferi = reg.interner_mut().intern("Teferi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(teferi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    let bounce_filter = ObjectFilter::permanent().with_types_any(TypeLine(
        TypeLine::ARTIFACT | TypeLine::CREATURE | TypeLine::ENCHANTMENT,
    ));

    reg.register(
        CardDefinition::new(name, chars)
            .with_cast_timing(CastTimingRestriction::OpponentsSorceryTiming)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, you may cast sorcery spells as \
                       though they had flash."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_sorceries_with_flash,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Return up to one target artifact, creature, or \
                       enchantment to its owner's hand. Draw a card."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(bounce_filter),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_bounce,
            }),
    )
}

fn plus_one_sorceries_with_flash(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::cast_as_though_flash_for(
            ctx.source,
            ctx.controller,
            ObjectFilter::default().with_types(TypeLine::SORCERY.into()),
            Duration::UntilYourNextTurn(ctx.controller),
        ),
    }]
}

fn minus_three_bounce(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() {
        effects.push(Effect::ReturnToHand { target: *id });
    }
    effects.push(Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    });
    effects
}
