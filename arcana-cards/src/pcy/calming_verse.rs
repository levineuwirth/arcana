//! Calming Verse — `{3}{G}` sorcery. "Destroy all enchantments you don't
//! control. If you control an untapped land, also destroy all enchantments
//! you control."
//!
//! The first clause (destroy opponent enchantments) is expressed via
//! ForEach + Opponent filter. The second clause requires a conditional
//! check for untapped land, which is not available via script:: helpers
//! (no untapped-permanent query).
//!
//! GAP: conditional check for controlling an untapped land (no
//! script::has_untapped_land or equivalent).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

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
                text: "Destroy all enchantments you don't control. If you control an untapped land, also destroy all enchantments you control.".into(),
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
    // Destroy all enchantments controlled by opponents
    let filter = ObjectFilter::new()
        .with_types(TypeLine::ENCHANTMENT.into())
        .controlled_by(ControllerConstraint::Opponent);
    let targets = script::ids_matching(state, &filter, entry.controller);
    let mut effects = vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }];
    // GAP: conditional "if you control an untapped land" (no script::has_untapped_land helper)
    // Second clause (destroy own enchantments if condition met) is omitted
    effects
}
