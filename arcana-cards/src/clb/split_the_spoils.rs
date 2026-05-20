//! Split the Spoils — `{2}{G}` sorcery. "Exile up to five target
//! permanent cards from your graveyard and separate them into two
//! piles. An opponent chooses one of those piles. Put that pile into
//! your hand and the other into your graveyard."
//!
//! The pile-split / opponent-chooses mechanic has no catalog Effect
//! (GAP'd). Best-effort: exile the targeted cards.

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
    let name = reg.interner_mut().intern("Split the Spoils");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile up to five target permanent cards from your graveyard and separate them into two piles. An opponent chooses one of those piles. Put that pile into your hand and the other into your graveyard. (Piles can be empty.)".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::permanent(),
                },
                count: TargetCount::UpTo(5),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: pile-split + opponent-chooses + per-pile hand/graveyard distribution not in catalog.
    let mut effects = Vec::new();
    for t in &entry.targets.targets {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::ExileFromGraveyard { target: *id });
        }
    }
    effects
}
