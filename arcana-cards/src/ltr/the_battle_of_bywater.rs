//! The Battle of Bywater — `{1}{W}{W}` sorcery. "Destroy all
//! creatures with power 3 or greater. Then create a Food token for
//! each creature you control."

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
    let name = reg.interner_mut().intern("The Battle of Bywater");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy all creatures with power 3 or greater. Then create a Food token for each creature you control.".into(),
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
    let big = script::ids_matching(
        state,
        &ObjectFilter::creature().with_min_power(3),
        entry.controller,
    );
    vec![Effect::ForEach {
        targets: big,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
    // GAP: "create a Food token for each creature you control" — Food
    // is an artifact token carrying an activated ability ("{2}, {T},
    // Sacrifice this token: You gain 3 life") which TokenDefinition
    // abilities cannot express with the demonstrated API.
}
