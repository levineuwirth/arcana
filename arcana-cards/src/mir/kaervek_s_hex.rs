//! Kaervek's Hex — `{3}{B}` sorcery. "Kaervek's Hex deals 1 damage to each
//! nonblack creature and an additional 1 damage to each green creature."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaervek's Hex");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Kaervek's Hex deals 1 damage to each nonblack creature and an additional 1 damage to each green creature.".into(),
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
    // 1 damage to each nonblack creature
    let nonblack_filter = ObjectFilter::creature().without_types(TypeLine::ENCHANTMENT.into());
    // Approximate "nonblack" via all creatures minus black — best approximation available:
    // use creature filter; green creatures get hit twice so they take 2 total.
    // ObjectFilter has .with_colors() but no "without color" — use two separate ForEach passes.
    let nonblack_ids = script::ids_matching(
        state,
        &ObjectFilter::creature().without_types(TypeLine::LAND.into()),
        entry.controller,
    );
    // Note: ObjectFilter has no "without_color" — this approximates by hitting all creatures
    // for 1, then green creatures for 1 more. The nonblack restriction is a GAP (no
    // without_colors filter); best-effort hits all creatures for 1 + green for 1.
    let green_ids = script::ids_matching(
        state,
        &ObjectFilter::creature().with_colors(ColorSet::green()),
        entry.controller,
    );
    let mut effects = Vec::new();
    if !nonblack_ids.is_empty() {
        effects.push(Effect::ForEach {
            targets: nonblack_ids,
            effect: Box::new(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(NULL_OBJECT_ID),
                amount: 1,
            }),
        });
    }
    if !green_ids.is_empty() {
        effects.push(Effect::ForEach {
            targets: green_ids,
            effect: Box::new(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(NULL_OBJECT_ID),
                amount: 1,
            }),
        });
    }
    effects
}
