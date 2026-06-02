//! All Suns' Dawn — `{4}{G}` sorcery, "For each color, return up to one
//! target card of that color from your graveyard to your hand. Exile All
//! Suns' Dawn."
//!
//! One up-to-one graveyard target per color (W/U/B/R/G), each
//! color-filtered and optional; every chosen card is returned to hand.
//! The self-exile rider has no expressible primitive (a spell exiling
//! itself on resolution is not in the catalog) — GAP'd below.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("All Suns' Dawn");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let colors = [
        ColorSet::white(),
        ColorSet::blue(),
        ColorSet::black(),
        ColorSet::red(),
        ColorSet::green(),
    ];
    let target_requirements = colors
        .into_iter()
        .map(|c| TargetRequirement {
            filter: TargetFilter::Card {
                zone: Zone::Graveyard(0),
                filter: ObjectFilter::default().with_colors(c),
            },
            count: TargetCount::UpTo(1),
            controller: None,
        })
        .collect();
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "For each color, return up to one target card of that color from your graveyard to your hand. Exile All Suns' Dawn.".into(),
                target_requirements,
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
    // GAP: self-exile of the resolving spell ("Exile All Suns' Dawn.")
    // has no expressible primitive.
    entry
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::ReturnFromGraveyardToHand { target: *id }),
            _ => None,
        })
        .collect()
}
