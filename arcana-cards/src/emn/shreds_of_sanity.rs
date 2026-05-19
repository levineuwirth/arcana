//! Shreds of Sanity — `{2}{R}` sorcery. "Return up to one target instant card and up
//! to one target sorcery card from your graveyard to your hand, then discard a card.
//! Exile Shreds of Sanity."
//!
//! # GAP: self-exile not expressible.

use arcana_core::effects::{Effect, DiscardChoice};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shreds of Sanity");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to one target instant card and up to one target sorcery card from your graveyard to your hand, then discard a card. Exile Shreds of Sanity.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::new().with_types(TypeLine::INSTANT.into()),
                        },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::new().with_types(TypeLine::SORCERY.into()),
                        },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
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
    // GAP: self-exile not expressible
    let mut effects = Vec::new();
    for t in &entry.targets.targets {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
        }
    }
    effects.push(Effect::Discard {
        player: entry.controller,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    });
    effects
}
