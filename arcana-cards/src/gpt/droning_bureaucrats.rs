//! Droning Bureaucrats — `{3}{W}` 1/4 white Human Advisor.
//! `{X}, {T}: Each creature with mana value X can't attack or block this turn.`
//!
//! The `{X}` cost fans out per affordable X; the resolver reads the paid X
//! from `ctx.x_value`, sweeps every battlefield creature whose mana value
//! equals X, and applies `ForbidAttacking` + `ForbidBlocking` for the turn.

use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::Effect;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Droning Bureaucrats");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}, {T}: Each creature with mana value X can't attack or block this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: forbid_attack_block,
            }),
    )
}

fn forbid_attack_block(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    let filter = ObjectFilter::creature().with_exact_cmc(x);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    let mut effects = Vec::with_capacity(ids.len() * 2);
    for id in ids {
        effects.push(Effect::ForbidAttacking {
            target: id,
            duration: Duration::EndOfTurn,
        });
        effects.push(Effect::ForbidBlocking {
            target: id,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}
