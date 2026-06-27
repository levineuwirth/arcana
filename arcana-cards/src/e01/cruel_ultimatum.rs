//! Cruel Ultimatum — `{U}{U}{B}{B}{B}{R}{R}` sorcery (the Grixis ultimatum).
//! "Target opponent sacrifices a creature, discards three cards, then loses 5
//!  life. You return a creature card from your graveyard to your hand, draw
//!  three cards, then gain 5 life."
//!
//! Fully faithful + always castable: only the opponent is a target (always
//! present in a 2-player game). The graveyard return is a resolution choice via
//! `ChooseNFromZone {min:1,max:1}`, which clamps to 0 when your graveyard holds
//! no creature — so an empty graveyard never blocks the cast and never fizzles.

use arcana_core::effects::{DiscardChoice, Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cruel Ultimatum");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}{B}{B}{B}{R}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent sacrifices a creature, discards three cards, then loses 5 life. You return a creature card from your graveyard to your hand, draw three cards, then gain 5 life.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let you = entry.controller;
    let Some(TargetChoice::Player(opp)) = entry.targets.targets.first() else { return Vec::new(); };
    let opp = *opp;
    vec![
        // Target opponent: sacrifice a creature (their choice), discard 3, lose 5.
        Effect::Sacrifice { player: opp, filter: ObjectFilter::creature(), count: 1 },
        Effect::Discard { player: opp, count: 3, choice: DiscardChoice::ControllerChooses },
        Effect::LoseLife { player: opp, amount: 5 },
        // You: return a creature from your graveyard, draw 3, gain 5.
        Effect::ChooseNFromZone {
            chooser: you,
            zone: Zone::Graveyard(you),
            filter: ObjectFilter::creature(),
            min: 1,
            max: 1,
            action: PickAction::ReturnToHand,
        },
        Effect::DrawCards { player: you, count: 3 },
        Effect::GainLife { player: you, amount: 5 },
    ]
}
