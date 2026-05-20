//! Reckless Crew — `{3}{R}` sorcery. "Create X 2/1 red Dwarf Berserker
//! creature tokens, where X is the number of Vehicles you control plus
//! the number of Equipment you control."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reckless Crew");
    let _b = reg.interner_mut().intern("Berserker");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create X 2/1 red Dwarf Berserker creature tokens, where X is the number of Vehicles you control plus the number of Equipment you control.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let vehicles = script::count_matching(
        state,
        &script::subtype_filter(reg, "Vehicle").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let equipment = script::count_matching(
        state,
        &script::subtype_filter(reg, "Equipment").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let x = vehicles + equipment;
    let berserker = reg.interner().lookup("Berserker").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(berserker);
    let token = TokenDefinition {
        name: berserker,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "for each of those tokens, you may attach an Equipment you
    // control" is not expressible.
    let mut out = Vec::new();
    for _ in 0..x {
        out.push(Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        });
    }
    out
}
