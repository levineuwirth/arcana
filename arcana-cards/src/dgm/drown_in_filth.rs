//! Drown in Filth — `{B}{G}` sorcery, "Choose target creature. Mill four
//! cards, then that creature gets -1/-1 until end of turn for each land
//! card in your graveyard."
//!
//! GAP: -X/-X where X = count of land cards in controller's graveyard
//! (no catalog variant for graveyard-count computed pump).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drown in Filth");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature. Mill four cards, then that creature gets -1/-1 until end of turn for each land card in your graveyard.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    // GAP: -X/-X where X = count of land cards in controller's graveyard
    vec![Effect::Mill { player: entry.controller, count: 4 }]
}
