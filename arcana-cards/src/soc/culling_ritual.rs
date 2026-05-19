//! Culling Ritual — `{2}{B}{G}` sorcery. "Destroy each nonland permanent with mana value 2 or
//! less. Add {B} or {G} for each permanent destroyed this way."
//! GAP: add-mana effect contingent on count of destroyed permanents not expressible.
//! Best effort: destroy all nonland permanents with CMC ≤ 2 via ForEach.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Culling Ritual");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy each nonland permanent with mana value 2 or less. Add {B} or {G} for each permanent destroyed this way.".into(),
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
    let filter = ObjectFilter::permanent()
        .without_types(TypeLine::LAND.into())
        .with_max_cmc(2);
    let targets = script::ids_matching(state, &filter, entry.controller);
    // GAP: add {B} or {G} for each destroyed permanent
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
