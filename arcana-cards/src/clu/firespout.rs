//! Firespout — `{2}{R/G}` sorcery. Colors: R, G.
//! "Firespout deals 3 damage to each creature without flying if {R} was spent
//! to cast this spell and 3 damage to each creature with flying if {G} was spent
//! to cast this spell."
//!
//! GAP: "if {R} was spent / if {G} was spent" — mana-spent conditional not in
//! engine catalog. Emitting the simpler approximation: deal 3 to all creatures
//! (which is correct when both {R} and {G} are spent, i.e. the common case for
//! the non-hybrid pip). The verify pipeline will flag this as partial.
//! Full correct implementation requires a "mana-spent" condition variant.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Firespout");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Firespout deals 3 damage to each creature without flying if {R} was spent to cast this spell and 3 damage to each creature with flying if {G} was spent to cast this spell.".into(),
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
    // GAP: mana-spent conditional ({R} vs {G}) not in engine catalog.
    // Approximation: deal 3 to all creatures (correct for {R}{G} spent case).
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 3,
        }),
    }]
}
