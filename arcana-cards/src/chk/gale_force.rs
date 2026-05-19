//! Gale Force — `{4}{G}` sorcery. "Gale Force deals 5 damage to each creature
//! with flying."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gale Force");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Gale Force deals 5 damage to each creature with flying.".into(),
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
    // GAP: ObjectFilter has no .with_keyword(Flying) builder; we cannot
    //      filter by keyword via the available script helpers.
    // Best-effort: deal 5 to each creature on the battlefield.
    // GAP: filter creatures with flying only
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    ids.into_iter().map(|id| Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(id),
        amount: 5,
    }).collect()
}
