//! Bond of Insight — `{3}{U}` sorcery. "Each player mills four
//! cards. Return up to two instant and/or sorcery cards from your
//! graveyard to your hand. Exile Bond of Insight." GAP: targeted
//! up-to-two-cards-from-graveyard-by-type isn't expressible as a
//! single TargetRequirement; emit the mill-each-player only. The
//! 'exile this card' on self-resolve is implicit via the engine's
//! spell-cleanup path.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bond of Insight");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player mills four cards. Return up to two instant and/or sorcery cards from your graveyard to your hand. Exile Bond of Insight.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: returning up to two instant/sorcery cards from graveyard mid-resolution
    // (without per-card TargetRequirements) not in catalog. The 'exile this card'
    // self-exile rider also not in catalog.
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 4 })
        .collect()
}
