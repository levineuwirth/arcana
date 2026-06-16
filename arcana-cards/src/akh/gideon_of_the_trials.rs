//! Gideon of the Trials — `{1}{W}{W}` Legendary Planeswalker — Gideon, starting loyalty 3.
//!
//! +1: Until your next turn, prevent all damage target permanent would deal.
//! 0: Until end of turn, Gideon becomes a 4/4 Human Soldier creature with
//!    indestructible that's still a planeswalker. Prevent all damage that would
//!    be dealt to him this turn.
//! 0: You get an emblem with "As long as you control a Gideon planeswalker, you
//!    can't lose the game and your opponents can't win the game."
//!
//! # Scope
//! - The `+1` (prevent-all-damage a target permanent would deal until your next
//!   turn) is a damage-prevention replacement the demonstrated `Effect` surface
//!   can't express — the ability shell keeps the correct `+1` cost and target,
//!   GAP'd effect body.
//! - The first `0` (becomes a 4/4 indestructible creature + damage prevention to
//!   itself) is a self-animation + prevention combination not expressible here —
//!   ability shell with `0` cost, GAP'd body.
//! - The second `0` is the EMBLEM. Its grant is a rule-altering static
//!   ("can't lose the game / opponents can't win") that the anthem/keyword/
//!   filtered builders cannot express — but the emblem is still CREATED (shell
//!   with the interned name + empty statics/abilities; GAP on the static text).

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gideon of the Trials");
    let gideon = reg.interner_mut().intern("Gideon");
    let _emblem = reg.interner_mut().intern("Gideon of the Trials emblem");
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
                text: "+1: Until your next turn, prevent all damage target \
                       permanent would deal."
                    .into(),
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
                text: "0: Until end of turn, Gideon of the Trials becomes a 4/4 \
                       Human Soldier creature with indestructible that's still a \
                       planeswalker. Prevent all damage that would be dealt to \
                       him this turn."
                    .into(),
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
                text: "0: You get an emblem with \"As long as you control a \
                       Gideon planeswalker, you can't lose the game and your \
                       opponents can't win the game.\""
                    .into(),
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

/// `+1: Until your next turn, prevent all damage target permanent would deal.`
fn plus_one_prevent(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: damage-prevention replacement effect (prevent all damage a target
    // permanent would deal) not expressible with the demonstrated Effect surface.
    Vec::new()
}

/// `0: Gideon becomes a 4/4 indestructible creature; prevent damage to him.`
fn zero_animate(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: planeswalker self-animation into a 4/4 indestructible creature plus
    // a self-targeted damage-prevention shield not expressible here.
    Vec::new()
}

/// `0: You get an emblem with "...can't lose the game / opponents can't win."`
fn zero_emblem(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Gideon of the Trials emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            // GAP: rule-altering static ("you can't lose the game and your
            // opponents can't win the game" while you control a Gideon
            // planeswalker) cannot be expressed by anthem/keyword/filtered
            // builders. Emblem shell is still created.
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
