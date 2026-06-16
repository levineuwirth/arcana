//! Gideon of the Trials — `{1}{W}{W}` Legendary Planeswalker — Gideon,
//! starting loyalty 3.
//!
//! +1: Until your next turn, prevent all damage target permanent would deal.
//! 0: Until end of turn, Gideon of the Trials becomes a 4/4 Human Soldier
//!    creature with indestructible that's still a planeswalker. Prevent all
//!    damage that would be dealt to him this turn.
//! 0: You get an emblem with "As long as you control a Gideon planeswalker,
//!    you can't lose the game and your opponents can't win the game."
//!
//! # Scope
//! - The `+1` damage-prevention rider ("prevent all damage target permanent
//!   would deal until your next turn") is a source-filtered prevention not
//!   cleanly expressible here; GAP'd body, correct `+1` cost + target shell.
//! - The first `0` is the SELF-ANIMATION (now supported): Gideon becomes a 4/4
//!   creature still a planeswalker, with indestructible, until end of turn. The
//!   self-targeted damage-prevention shield is GAP'd; the animation IS applied.
//! - The second `0` is the EMBLEM (created): its grant is a rule-altering static
//!   ("can't lose / opponents can't win") the anthem/keyword/filtered builders
//!   can't express, so the emblem shell is created with empty statics/abilities.

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility};
use arcana_core::layers::Duration;
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
    // GAP: "prevent all damage a target permanent would deal until your next
    // turn" is a source-filtered prevention not expressible with the
    // demonstrated Effect surface. Correct +1 cost + target shell retained.
    Vec::new()
}

/// `0: Gideon becomes a 4/4 indestructible creature, still a planeswalker.`
fn zero_animate(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // Self-animation (now supported): AddType is additive so the planeswalker
    // types are kept; SetBasePT makes it a 4/4; indestructible granted. The
    // self-targeted damage-prevention shield is GAP'd (prevention surface).
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 4,
            toughness: 4,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
    ]
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
            // opponents can't win the game") cannot be expressed by the
            // anthem/keyword/filtered builders. Emblem shell still created.
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
