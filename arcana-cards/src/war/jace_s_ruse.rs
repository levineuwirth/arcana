//! Jace's Ruse — `{3}{U}{U}` sorcery. "Return up to two target creatures to their owner's hand.
//! You may search your library and/or graveyard for a card named Jace, Arcane Strategist, reveal it,
//! and put it into your hand. If you search your library this way, shuffle."
//! GAP: searching graveyard by exact card name and multi-zone named tutor not in catalog.
//! Best effort: return up to two creatures; tutor part is GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jace's Ruse");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to two target creatures to their owner's hand. You may search your library and/or graveyard for a card named Jace, Arcane Strategist, reveal it, and put it into your hand. If you search your library this way, shuffle.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
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
    // GAP: named-card tutor from library and/or graveyard not in catalog
    entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::ReturnToHand { target: *id })
        } else {
            None
        }
    }).collect()
}
