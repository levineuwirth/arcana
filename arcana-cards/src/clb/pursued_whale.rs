//! Pursued Whale — `{5}{U}{U}` 8/8 Whale.
//!
//! Oracle:
//! * When this creature enters, each opponent creates a 1/1 red Pirate
//!   creature token with "This token can't block" and "Creatures you
//!   control attack each combat if able." — ETB trigger; one
//!   `CreateToken` per opponent (a 1/1 red Pirate). The "attack each
//!   combat if able" static is wired as a token-borne
//!   SelfEntersBattlefield trigger installing a board-wide
//!   `filtered_must_attack`. GAP: "This token can't block" has no
//!   attached/static can't-block restriction primitive.
//! * Spells your opponents cast that target this creature cost {3} more
//!   to cast. — GAP: static cost-increase replacement effect, not a
//!   triggered/activated ability and not in this surface.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pursued Whale");
    let whale = reg.interner_mut().intern("Whale");
    let _pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(whale);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };

    // GAP: static "Spells your opponents cast that target this creature cost {3}
    // more to cast" — a cost-increase replacement effect, not expressible here.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: each_opponent_makes_pirate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn each_opponent_makes_pirate(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let pirate = reg.interner().lookup("Pirate").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pirate);
    // GAP: token's "This token can't block" — no attached/static can't-block
    // restriction primitive. The "Creatures you control attack each combat if
    // able" static is wired as the token-borne trigger below.
    let opponents = script::opponents(state, trig.controller);
    opponents
        .into_iter()
        .map(|opp| Effect::CreateToken {
            controller: opp,
            token: TokenDefinition {
                name: pirate,
                colors: ColorSet::red(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![TriggeredAbilityDef {
                    id: 1,
                    trigger_condition: TriggerCondition::SelfEntersBattlefield,
                    intervening_if: None,
                    effect: token_install_must_attack,
                    trigger_zones: vec![Zone::Battlefield],
                    frequency: TriggerFrequency::EachTime,
                    target_requirements: Vec::new(),
                }],
            },
        })
        .collect()
}

fn token_install_must_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Creatures you control attack each combat if able." The token is the
    // source; FilteredMustAttack reads the source's controller, so
    // controlled_by(You) = creatures the token's controller controls.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_must_attack(
            trig.source,
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
