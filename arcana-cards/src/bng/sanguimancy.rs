//! Sanguimancy — `{4}{B}` sorcery. "You draw X cards and you lose X
//! life, where X is your devotion to black." X is computed at
//! resolution via `script::devotion` (count of {B} pips among permanents
//! you control, CR 700.5).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sanguimancy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You draw X cards and you lose X life, where X is your devotion to black.".into(),
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
    let x = script::devotion(state, entry.controller, ColorSet::black());
    vec![
        Effect::DrawCards { player: entry.controller, count: x },
        Effect::LoseLife { player: entry.controller, amount: x },
    ]
}
