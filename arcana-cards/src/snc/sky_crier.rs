//! Sky Crier — `{1}{W}` 1/1 Bird Citizen.
//!
//! Oracle:
//! * Flying, lifelink
//! * `{3}{W}: You and target opponent each draw a card.`
//!
//! Flying/lifelink are base keywords. The activated ability targets an
//! opponent (a player) and draws for both that player and the controller.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sky Crier");
    let bird = reg.interner_mut().intern("Bird");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars).with_activated_ability(
        ActivatedAbilityDef {
            text: "{3}{W}: You and target opponent each draw a card.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::Exactly(1),
                controller: Some(ControllerConstraint::Opponent),
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: each_draw_a_card,
        },
    ))
}

fn each_draw_a_card(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }];
    if let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() {
        effects.push(Effect::DrawCards {
            player: *p,
            count: 1,
        });
    }
    effects
}
