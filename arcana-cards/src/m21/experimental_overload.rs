//! Experimental Overload — `{2}{U}{R}` sorcery. "Create an X/X blue
//! and red Weird creature token, where X is the number of instant and
//! sorcery cards in your graveyard. Then you may return an instant or
//! sorcery card from your graveyard to your hand. Exile Experimental
//! Overload." The optional return and self-exile have no primitives;
//! the dynamic-P/T token is emitted.

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
    let name = reg.interner_mut().intern("Experimental Overload");
    let _weird = reg.interner_mut().intern("Weird");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create an X/X blue and red Weird creature token, where X is the number of instant and sorcery cards in your graveyard. Then you may return an instant or sorcery card from your graveyard to your hand. Exile Experimental Overload.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let weird = reg.interner().lookup("Weird").expect("Weird interned");
    let x = script::graveyard_matching(
        state,
        &ObjectFilter::new()
            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
        entry.controller,
        entry.controller,
    ) as i32;
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(weird);
    let token = TokenDefinition {
        name: weird,
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(x)),
        toughness: Some(PtValue::Fixed(x)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: optional graveyard return and self-exile have no primitive.
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
