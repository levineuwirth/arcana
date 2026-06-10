//! Chimeric Coils — `{1}` artifact.
//! "{X}{1}: This artifact becomes an X/X Construct artifact creature.
//! Sacrifice it at the beginning of the next end step."
//! Self-animation: AddType(CREATURE) + SetBasePT(X/X) while on the
//! battlefield, plus a delayed sacrifice at the next end step.
//! GAP: the Construct creature subtype cannot be added (AddType covers
//! card types only — no subtype-add effect).

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chimeric Coils");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{X}{1}: This artifact becomes an X/X Construct artifact creature. Sacrifice it at the beginning of the next end step.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{X}{1}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: animate_self,
        }),
    )
}

fn animate_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0) as i32;
    // GAP: 'Construct' subtype not addable (AddType is card types only).
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: x,
            toughness: x,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::DelayedAction {
            source: ctx.source,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Sacrifice,
        },
    ]
}
