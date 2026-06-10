//! Forge of Heroes — nonbasic land (Commander 2018).
//! "{T}: Add {C}." and "{T}: Choose target commander that entered
//! this turn. Put a +1/+1 counter on it if it's a creature and a
//! loyalty counter on it if it's a planeswalker."
//!
//! GAP: commander status is not modeled (no commander predicate on
//! `ObjectFilter`), nor is "entered this turn" as a target
//! restriction, nor a type-dependent counter choice — the utility
//! activation targets a permanent and its effect is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forge of Heroes");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Choose target commander that entered this turn. \
                       Put a +1/+1 counter on it if it's a creature and a \
                       loyalty counter on it if it's a planeswalker."
                    .into(),
                cost: ActivationCost::tap_only(),
                // GAP: 'target commander that entered this turn' —
                // commander status and entered-this-turn target
                // restrictions are not expressible; broad permanent
                // target declared.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::new()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: empower_commander,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn empower_commander(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: type-dependent counter choice ('+1/+1 if creature, loyalty
    // if planeswalker') on an unmodeled commander target — effect
    // GAP'd.
    Vec::new()
}
