//! Vraska's Stoneglare — `{4}{B}{G}` sorcery, "Destroy target creature.
//! You gain life equal to its toughness. Search your library for a card
//! named Vraska, Golgari Queen, reveal it, put it into your hand, then
//! shuffle."
//!
//! GAP: life gain = toughness of destroyed creature; named-card tutor.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vraska's Stoneglare");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature. You gain life equal to its toughness. Search your library for a card named Vraska, Golgari Queen, reveal it, put it into your hand, then shuffle.".into(),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: life gain = toughness of destroyed creature; named-card tutor
    vec![Effect::DestroyPermanent { target: *id }]
}
