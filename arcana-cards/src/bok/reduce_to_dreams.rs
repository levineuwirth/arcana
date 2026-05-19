//! Reduce to Dreams — `{3}{U}{U}` sorcery.
//! "Return all artifacts and enchantments to their owners' hands."

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
    let name = reg.interner_mut().intern("Reduce to Dreams");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return all artifacts and enchantments to their owners' hands.".into(),
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
    let artifacts = script::ids_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        entry.controller,
    );
    let enchantments = script::ids_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()),
        entry.controller,
    );
    let mut all = artifacts;
    all.extend(enchantments);
    vec![Effect::ForEach {
        targets: all,
        effect: Box::new(Effect::ReturnToHand { target: arcana_core::objects::NULL_OBJECT_ID }),
    }]
}
