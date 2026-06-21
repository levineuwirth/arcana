//! Goblin Wizard — `{2}{R}{R}` 1/1 Creature — Goblin Wizard. Mono-red.
//!
//! Oracle:
//! - "{T}: You may put a Goblin permanent card from your hand onto the
//!   battlefield." — activated, `PutFromHandOntoBattlefield` over a Goblin
//!   subtype filter.
//! - "{R}: Target Goblin gains protection from white until end of turn." —
//!   GAP: Protection is not an expressible `KeywordAbility` / `Effect`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Wizard");
    let goblin = reg.interner_mut().intern("Goblin");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: You may put a Goblin permanent card from your hand onto the battlefield.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_goblin_from_hand,
            })
            // GAP: "{R}: Target Goblin gains protection from white until end of
            // turn." — Protection is not an expressible effect/keyword.
    )
}

fn put_goblin_from_hand(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Goblin").controlled_by(ControllerConstraint::You);
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter,
        tapped: false,
    }]
}
