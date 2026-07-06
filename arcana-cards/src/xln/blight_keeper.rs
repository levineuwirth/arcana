//! Blight Keeper — `{B}` 1/1 black Bat Imp with Flying.
//! "{7}{B}, {T}, Sacrifice this creature: Target opponent loses 4 life
//! and you gain 4 life."
//!
//! Abilities:
//! 1. Flying (keyword).
//! 2. {7}{B}, {T}, Sacrifice ~: target opponent loses 4 life and you
//!    gain 4 life.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blight Keeper");
    let bat = reg.interner_mut().intern("Bat");
    let imp = reg.interner_mut().intern("Imp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);
    subtypes.0.insert(imp);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{7}{B}, {T}, Sacrifice this creature: Target opponent loses 4 life \
                   and you gain 4 life."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{7}{B}").unwrap(),
                tap: true,
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_opponent()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: drain_opponent,
        }),
    )
}

fn drain_opponent(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![
        Effect::LoseLife {
            player: *p,
            amount: 4,
        },
        Effect::GainLife {
            player: ctx.controller,
            amount: 4,
        },
    ]
}
