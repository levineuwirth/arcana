//! Questing Phelddagrif — `{1}{G}{W}{U}` 4/4 Phelddagrif (G/W/U).
//!
//! Three pip-activated abilities, each pumping/buffing this creature with a
//! symmetric upside for a target opponent:
//! * "{G}: This creature gets +1/+1 until end of turn. Target opponent creates a
//!   1/1 green Hippo creature token." → Pump self + CreateToken (Hippo) for the
//!   opponent.
//! * "{W}: This creature gains protection from black and from red until end of
//!   turn. Target opponent gains 2 life." → GainLife for the opponent.
//!   GAP: granting protection-from-color is not in the usable keyword/effect
//!   surface; only the opponent's life gain is wired.
//! * "{U}: This creature gains flying until end of turn. Target opponent may draw
//!   a card." → GrantKeyword Flying on self + DrawCards for the opponent (the
//!   "may" is simplified to an unconditional draw).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Questing Phelddagrif");
    let phelddagrif = reg.interner_mut().intern("Phelddagrif");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phelddagrif);
    // Pre-intern the Hippo token's subtype/name so the resolver can look it up.
    reg.interner_mut().intern("Hippo");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let opp_target = || TargetRequirement {
        filter: TargetFilter::Player,
        count: TargetCount::Exactly(1),
        controller: Some(ControllerConstraint::Opponent),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}: This creature gets +1/+1 until end of turn. Target opponent creates a 1/1 green Hippo creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![opp_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: green_ability,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}: This creature gains protection from black and from red until end of turn. Target opponent gains 2 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![opp_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: white_ability,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}: This creature gains flying until end of turn. Target opponent may draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![opp_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: blue_ability,
            }),
    )
}

fn green_ability(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let mut out = vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    if let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() {
        if let Some(hippo) = reg.interner().lookup("Hippo") {
            let mut subtypes = SubtypeSet::default();
            subtypes.0.insert(hippo);
            out.push(Effect::CreateToken {
                controller: *p,
                token: TokenDefinition {
                    name: hippo,
                    colors: ColorSet::green(),
                    types: TypeLine::CREATURE.into(),
                    subtypes,
                    power: Some(PtValue::Fixed(1)),
                    toughness: Some(PtValue::Fixed(1)),
                    keywords: vec![],
                    abilities: vec![],
                },
            });
        }
    }
    out
}

fn white_ability(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "this creature gains protection from black and from red" — protection
    // is not in the usable keyword/effect surface.
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::GainLife { player: *p, amount: 2 }]
}

fn blue_ability(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut out = vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Flying,
        duration: Duration::EndOfTurn,
    }];
    if let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() {
        // "may draw a card" simplified to an unconditional draw for the opponent.
        out.push(Effect::DrawCards { player: *p, count: 1 });
    }
    out
}
