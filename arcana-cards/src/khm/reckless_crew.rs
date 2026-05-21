//! Reckless Crew — `{3}{R}` sorcery. "Create X 2/1 red Dwarf Berserker
//! creature tokens, where X is the number of Vehicles you control plus
//! the number of Equipment you control. For each of those tokens, you
//! may attach an Equipment you control to it." X is dynamic
//! (Vehicles + Equipment you control); the per-token attach is a GAP
//! (no catalog primitive for runtime equipment attach).

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
    let name = reg.interner_mut().intern("Reckless Crew");
    let _dwarf = reg.interner_mut().intern("Dwarf");
    let _berserker = reg.interner_mut().intern("Berserker");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create X 2/1 red Dwarf Berserker creature tokens, where X is the number of Vehicles you control plus the number of Equipment you control. For each of those tokens, you may attach an Equipment you control to it.".into(),
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
    let dwarf = reg
        .interner()
        .lookup("Dwarf")
        .expect("Dwarf interned during register()");
    let berserker = reg
        .interner()
        .lookup("Berserker")
        .expect("Berserker interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(berserker);
    let vehicles = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let equipment = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    // GAP: cannot filter by 'Vehicle' or 'Equipment' subtype here
    // without a subtype filter helper for arbitrary subtypes — falls
    // back to all artifacts you control, which is approximate.
    let x = vehicles + equipment;
    let token = TokenDefinition {
        name: dwarf,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = Vec::with_capacity(x as usize);
    for _ in 0..x {
        effects.push(Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        });
    }
    // GAP: 'for each of those tokens, you may attach an Equipment you
    // control' has no catalog primitive for runtime equip.
    effects
}
