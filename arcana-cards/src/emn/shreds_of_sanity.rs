//! Shreds of Sanity — `{2}{R}` sorcery. "Return up to one target
//! instant card and up to one target sorcery card from your
//! graveyard to your hand, then discard a card. Exile Shreds of
//! Sanity." The self-exile has no primitive; the returns and discard
//! are modeled (partial).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::zones::Zone;
use arcana_core::types::{CardId, ColorSet, TypeLine};

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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return up to one target instant card and up to one target sorcery card from your graveyard to your hand, then discard a card. Exile Shreds of Sanity.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new()
                            .with_types(TypeLine::INSTANT.into()),
                    },
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new()
                            .with_types(TypeLine::SORCERY.into()),
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

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Exile Shreds of Sanity" (self-exile on resolution) has no
    // primitive; the returns and discard are modeled.
    let mut effects: Vec<Effect> = entry
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => {
                Some(Effect::ReturnFromGraveyardToHand { target: *id })
            }
            _ => None,
        })
        .collect();
    effects.push(Effect::Discard {
        player: entry.controller,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    });
    effects
}
