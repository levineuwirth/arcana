//! Brilliant Restoration — `{3}{W}{W}{W}{W}` sorcery.
//! "Return all artifact and enchantment cards from your graveyard to the
//! battlefield."
//!
//! Collects artifact-or-enchantment card IDs from the controller's graveyard
//! and returns each to the battlefield via `Effect::ForEach`.

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
    let name = reg.interner_mut().intern("Brilliant Restoration");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return all artifact and enchantment cards from your graveyard to the battlefield.".into(),
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
    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT));
    let ids = script::graveyard_ids_matching(state, &filter, entry.controller, entry.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ReturnFromGraveyardToBattlefield {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
