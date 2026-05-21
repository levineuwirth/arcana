//! Miming Slime — `{2}{G}` sorcery. "Create an X/X green Ooze
//! creature token, where X is the greatest power among creatures you
//! control."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Miming Slime");
    let _ooze = reg.interner_mut().intern("Ooze");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create an X/X green Ooze creature token, where X is the greatest power among creatures you control.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let ooze = reg
        .interner()
        .lookup("Ooze")
        .expect("Ooze interned during register()");
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let x = ids
        .iter()
        .map(|id| script::power_of(state, *id))
        .max()
        .unwrap_or(0)
        .max(0);
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);
    let token = TokenDefinition {
        name: ooze,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(x)),
        toughness: Some(PtValue::Fixed(x)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: entry.controller,
        token,
    }]
}
