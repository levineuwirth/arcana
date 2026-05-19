//! Cosmic Epiphany — `{4}{U}{U}` sorcery.
//! "Draw cards equal to the number of instant and sorcery cards in your graveyard."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cosmic Epiphany");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw cards equal to the number of instant and sorcery cards in your graveyard.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    let n = script::graveyard_matching(state, &filter, entry.controller, entry.controller);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards { player: entry.controller, count: n }]
}
