//! Bontu's Last Reckoning — `{1}{B}{B}` sorcery. "Destroy all
//! creatures. Lands you control don't untap during your next untap
//! step." The land-skip-untap rider isn't in the catalog; we emit the
//! wipe and GAP the rider.

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
    let name = reg.interner_mut().intern("Bontu's Last Reckoning");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures. Lands you control don't untap during your next untap step.".into(),
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
    // GAP: 'lands you control don't untap next untap step' delayed
    // restriction not in catalog.
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    ids.into_iter().map(|id| Effect::DestroyPermanent { target: id }).collect()
}
