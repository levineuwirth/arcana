//! Soul Conduit — `{6}` artifact (Shadowmoor).
//! "{6}, {T}: Two target players exchange life totals."
//!
//! The exchange is computed at resolution: read both players' life via
//! `script::life`, then set each to the other's total.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul Conduit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{6}, {T}: Two target players exchange life totals."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exchange_life_totals,
            },
        ),
    )
}

fn exchange_life_totals(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let targets = &ctx.targets.targets;
    let (Some(TargetChoice::Player(a)), Some(TargetChoice::Player(b))) =
        (targets.first(), targets.get(1))
    else {
        return Vec::new();
    };
    let a_life = script::life(state, *a);
    let b_life = script::life(state, *b);
    vec![
        Effect::SetLifeTotal { player: *a, amount: b_life.max(0) as u32 },
        Effect::SetLifeTotal { player: *b, amount: a_life.max(0) as u32 },
    ]
}
