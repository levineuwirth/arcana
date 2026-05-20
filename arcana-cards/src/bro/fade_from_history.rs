//! Fade from History — `{2}{G}{G}` sorcery, "Each player who controls
//! an artifact or enchantment creates a 2/2 green Bear creature token.
//! Then destroy all artifacts and enchantments." The conditional
//! per-player token (gated on whether that player controls an
//! artifact/enchantment) is not expressible; the board wipe is.

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
    let name = reg.interner_mut().intern("Fade from History");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each player who controls an artifact or enchantment \
                   creates a 2/2 green Bear creature token. Then destroy \
                   all artifacts and enchantments."
                .into(),
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
    // GAP: the per-player conditional Bear token (created only by
    // players controlling an artifact/enchantment) is not expressible.
    let mut ids = script::ids_matching(
        state,
        &ObjectFilter::permanent().with_types_any(TypeLine::ARTIFACT.into()),
        entry.controller,
    );
    ids.extend(script::ids_matching(
        state,
        &ObjectFilter::permanent()
            .with_types_any(TypeLine::ENCHANTMENT.into()),
        entry.controller,
    ));
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
