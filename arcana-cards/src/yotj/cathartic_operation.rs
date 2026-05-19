//! Cathartic Operation — `{3}{U}{B}` instant. "Return up to two target
//! creature cards from your graveyard to your hand, then seek two
//! noncreature, nonland cards."
//!
//! # GAP: Seek mechanic (take a random matching card from library without
//! revealing) has no Effect variant. Best-effort: graveyard returns only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::zones::Zone;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cathartic Operation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to two target creature cards from your graveyard to your hand, \
                       then seek two noncreature, nonland cards."
                    .into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::UpTo(2),
                    controller: None,
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
    // GAP: Seek mechanic not expressible
    entry
        .targets
        .targets
        .iter()
        .filter_map(|t| {
            if let TargetChoice::Object(id) = t {
                Some(Effect::ReturnFromGraveyardToHand { target: *id })
            } else {
                None
            }
        })
        .collect()
}
