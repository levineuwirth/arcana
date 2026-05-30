//! Anurid Brushhopper — `{1}{G}{W}` 3/4 Frog Beast.
//! `Discard two cards: Exile this creature. Return it to the battlefield
//! under its owner's control at the beginning of the next end step.`

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anurid Brushhopper");
    let frog = reg.interner_mut().intern("Frog");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Discard two cards: Exile this creature. Return it to the battlefield under its owner's control at the beginning of the next end step.".into(),
                cost: ActivationCost {
                    discard_other: Some(ObjectFilter::default()),
                    discard_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: phase_out_self,
            }),
    )
}

fn phase_out_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::ExilePermanent { target: ctx.source },
        Effect::DelayedAction {
            source: ctx.source,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnFromExileToBattlefield,
        },
    ]
}
