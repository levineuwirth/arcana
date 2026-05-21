//! Kin-Tree Invocation — `{B}{G}` sorcery. "Create an X/X black and
//! green Spirit Warrior creature token, where X is the greatest
//! toughness among creatures you control." 'Greatest toughness among
//! creatures you control' isn't directly in the script surface — we
//! can iterate ids_matching(creatures-you-control) and read
//! toughness_of for each, taking the max.

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
    let name = reg.interner_mut().intern("Kin-Tree Invocation");
    let _spirit = reg.interner_mut().intern("Spirit");
    let _warrior = reg.interner_mut().intern("Warrior");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create an X/X black and green Spirit Warrior creature token, where X is the greatest toughness among creatures you control.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let mut x: i32 = 0;
    for id in &ids {
        let t = script::toughness_of(state, *id);
        if t > x { x = t; }
    }
    let xv = x.max(0);
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned during register()");
    let warrior = reg.interner().lookup("Warrior").expect("Warrior interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(xv)),
        toughness: Some(PtValue::Fixed(xv)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
