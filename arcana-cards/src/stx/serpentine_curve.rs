//! Serpentine Curve — `{3}{U}` sorcery. "Create a 0/0 green and blue Fractal
//! creature token. Put X +1/+1 counters on it, where X is one plus the total
//! number of instant and sorcery cards you own in exile and in your graveyard."
//!
//! GAP: counting cards in exile zone — script::graveyard_matching covers
//! graveyard but no equivalent for exile zone.
//! GAP: AddCounters on a just-created token (no handle to the new token's ObjectId).
//! Best-effort: create the 0/0 token; counter placement GAP noted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serpentine Curve");
    let _fractal = reg.interner_mut().intern("Fractal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a 0/0 green and blue Fractal creature token. Put X +1/+1 counters on it, where X is one plus the total number of instant and sorcery cards you own in exile and in your graveyard.".into(),
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
    // GAP: no script helper for counting cards in exile
    // GAP: no handle to newly created token's ObjectId for AddCounters
    let gy_instants = script::graveyard_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::INSTANT.into()),
        entry.controller,
        entry.controller,
    );
    let gy_sorceries = script::graveyard_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::SORCERY.into()),
        entry.controller,
        entry.controller,
    );
    let _x = 1 + gy_instants + gy_sorceries; // exile count not available
    let fractal = reg.interner().lookup("Fractal").expect("Fractal interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fractal);
    let token = TokenDefinition {
        name: fractal,
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: cannot AddCounters on just-created token (no ObjectId handle)
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
