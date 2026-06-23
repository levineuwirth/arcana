//! Charix, the Raging Isle — `{2}{U}{U}` 0/17 Legendary Leviathan Crab.
//!
//! Oracle:
//! Spells your opponents cast that target Charix cost {2} more to cast.
//! {3}: Charix gets +X/-X until end of turn, where X is the number of
//!      Islands you control.
//!
//! Decomposition:
//! - Static cost-increase on opponents' targeting spells — no cost-tax
//!   effect/static hook in the demonstrated API, GAP'd.
//! - `{3}` activated +X/-X where X = Islands you control — wired as an
//!   activated ability whose resolver computes X from the board.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Charix, the Raging Isle");
    let leviathan = reg.interner_mut().intern("Leviathan");
    let crab = reg.interner_mut().intern("Crab");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leviathan);
    subtypes.0.insert(crab);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(17)),
        ..Default::default()
    };

    // GAP (static): "Spells your opponents cast that target Charix cost {2}
    // more to cast" — no spell-cost-tax static hook in the demonstrated API.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}: Charix, the Raging Isle gets +X/-X until end of turn, where X is the number of Islands you control.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_by_islands,
        }),
    )
}

fn pump_by_islands(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Island").controlled_by(ControllerConstraint::You);
    let x = script::count_matching(state, &filter, ctx.controller) as i32;
    vec![Effect::Pump {
        target: ctx.source,
        power: x,
        toughness: -x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
