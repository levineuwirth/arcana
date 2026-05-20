//! Vraska's Stoneglare — `{4}{B}{G}` sorcery. "Destroy target
//! creature. You gain life equal to its toughness. You may search
//! your library and/or graveyard for a card named Vraska, Regal
//! Gorgon, reveal it, and put it into your hand."
//!
//! The named-card tutor from library/graveyard is not expressible;
//! the destroy + lifegain-equal-to-toughness portion is emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target creature. You gain life equal to its toughness. You may search your library and/or graveyard for a card named Vraska, Regal Gorgon, reveal it, and put it into your hand. If you search your library this way, shuffle.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let tough = script::toughness_of(state, *id).max(0) as u32;
    // GAP: search for a specifically-named card in library/graveyard
    // is not expressible.
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::GainLife {
            player: entry.controller,
            amount: tough,
        },
    ]
}
