//! Gideon of the Trials — `{1}{W}{W}` Legendary Planeswalker — Gideon,
//! starting loyalty 3.
//!
//! +1: Until your next turn, prevent all damage target permanent would
//!     deal. (GAP — prevention keyed to a specific source object; the
//!     prevention surface filters sources by ObjectFilter, not by a chosen
//!     target id.)
//! 0: Until end of turn, Gideon becomes a 4/4 Human Soldier creature with
//!    indestructible that's still a planeswalker. Prevent all damage that
//!    would be dealt to him this turn. (GAP — planeswalker animation
//!    bundle + self damage prevention.)
//! 0: You get an emblem with "As long as you control a Gideon planeswalker,
//!    you can't lose the game and your opponents can't win the game."
//!    (GAP — emblem with a game-rule-altering static.)

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gideon of the Trials");
    let gideon = reg.interner_mut().intern("Gideon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gideon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, prevent all damage target permanent would deal.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_prevent,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Until end of turn, Gideon becomes a 4/4 Human Soldier creature with indestructible that's still a planeswalker. Prevent all damage that would be dealt to him this turn.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_animate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: You get an emblem with \"As long as you control a Gideon planeswalker, you can't lose the game and your opponents can't win the game.\"".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_emblem,
            }),
    )
}

fn plus_one_prevent(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "prevent all damage target permanent would deal" — the prevention
    //      surface (PreventDamageFrom) filters sources by ObjectFilter, not
    //      by a single chosen target object id, so a per-target source
    //      prevention can't be installed here.
    Vec::new()
}

fn zero_animate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: planeswalker-animation (becomes a 4/4 indestructible creature,
    //      still a planeswalker) bundled with self damage prevention — not
    //      expressible from the demonstrated surface.
    Vec::new()
}

fn zero_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with a game-rule-altering static ("you can't lose / your
    //      opponents can't win"). EmblemDefinition carries only triggered
    //      abilities; this static can't be expressed.
    Vec::new()
}
