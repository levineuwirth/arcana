//! Calming Verse — `{3}{G}` sorcery. "Destroy all enchantments you
//! don't control. Then if you control an untapped land, destroy all
//! enchantments you control."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Calming Verse");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all enchantments you don't control. Then if you control an untapped land, destroy all enchantments you control.".into(),
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
    let theirs = script::ids_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ENCHANTMENT.into())
            .controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );
    let mut effects = vec![Effect::ForEach {
        targets: theirs,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }];
    let untapped_lands = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You)
            .untapped_only(),
        entry.controller,
    );
    if untapped_lands > 0 {
        let mine = script::ids_matching(
            state,
            &ObjectFilter::permanent()
                .with_types(TypeLine::ENCHANTMENT.into())
                .controlled_by(ControllerConstraint::You),
            entry.controller,
        );
        effects.push(Effect::ForEach {
            targets: mine,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        });
    }
    effects
}
