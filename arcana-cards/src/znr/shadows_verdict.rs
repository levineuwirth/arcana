//! Shadows' Verdict — `{3}{B}{B}` sorcery. "Exile all creatures and
//! planeswalkers with mana value 3 or less from the battlefield and all
//! creature and planeswalker cards with mana value 3 or less from all
//! graveyards."
//!
//! GAP: planeswalker type filter not expressible. GAP: exile from all
//! graveyards not expressible (only controller's graveyard accessible).
//! Approximated as exile all creatures with CMC ≤ 3 on the battlefield.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shadows' Verdict");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile all creatures and planeswalkers with mana value 3 or less from the battlefield and all creature and planeswalker cards with mana value 3 or less from all graveyards.".into(),
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
    // GAP: planeswalker filter not expressible
    // GAP: exile from all graveyards not expressible
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().with_max_cmc(3),
        entry.controller,
    );
    if ids.is_empty() { return Vec::new(); }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
    }]
}
