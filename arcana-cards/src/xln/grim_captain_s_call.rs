//! Grim Captain's Call — `{2}{B}` sorcery, "Return a Pirate card from your
//! graveyard to your hand, then do the same for Vampire, Dinosaur, and
//! Merfolk." Four sequential graveyard-to-hand tutors filtered by creature
//! subtype; the subtype filter is applied per the CARD SCRIPTING helpers.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grim Captain's Call");
    let _pirate = reg.interner_mut().intern("Pirate");
    let _vampire = reg.interner_mut().intern("Vampire");
    let _dinosaur = reg.interner_mut().intern("Dinosaur");
    let _merfolk = reg.interner_mut().intern("Merfolk");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return a Pirate card from your graveyard to your hand, then do the same for Vampire, Dinosaur, and Merfolk.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::creature(),
                        },
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::creature(),
                        },
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::creature(),
                        },
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::creature(),
                        },
                        count: TargetCount::Exactly(1),
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
    entry.targets.targets.iter().filter_map(|t| {
        use arcana_core::targets::TargetChoice;
        if let TargetChoice::Object(id) = t {
            Some(Effect::ReturnFromGraveyardToHand { target: *id })
        } else {
            None
        }
    }).collect()
}
