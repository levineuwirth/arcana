//! Mirror Universe — `{6}` artifact (Legends).
//! "{T}, Sacrifice this artifact: Exchange life totals with target
//! opponent. Activate only during your upkeep."
//!
//! The exchange is modeled by reading both life totals at resolution and
//! setting each player to the other's total. GAP: the "Activate only
//! during your upkeep" timing window and the opponent-only target
//! restriction.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mirror Universe");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice this artifact: Exchange life totals with target opponent. Activate only during your upkeep.".into(),
            cost: ActivationCost {
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
            effect: exchange_life_totals,
        }),
    )
}

fn exchange_life_totals(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Activate only during your upkeep" — no activation timing
    // window is expressible.
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let mine = script::life(state, ctx.controller).max(0) as u32;
    let theirs = script::life(state, *p).max(0) as u32;
    vec![
        Effect::SetLifeTotal { player: ctx.controller, amount: theirs },
        Effect::SetLifeTotal { player: *p, amount: mine },
    ]
}
