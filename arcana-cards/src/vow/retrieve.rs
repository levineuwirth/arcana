//! Retrieve — `{2}{G}` sorcery. Return up to one target creature card and up
//! to one target noncreature permanent card from your graveyard to your hand.
//! Exile Retrieve.
//!
//! GAP: "exile this spell" (self-exile from stack/graveyard) not in catalog;
//! the two `ReturnFromGraveyardToHand` effects are expressible with UpTo(1)
//! targets each.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Retrieve");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return up to one target creature card and up to one target noncreature permanent card from your graveyard to your hand. Exile Retrieve.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::permanent().without_types(TypeLine::CREATURE.into()),
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
    let mut effects = Vec::new();
    for target in &entry.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
        }
    }
    // GAP: exile Retrieve itself (self-exile from stack not expressible)
    effects
}
