//! Elemental Spectacle — `{5}{G}` sorcery. "Vivid — Create a number of 5/5
//! red and green Elemental creature tokens equal to the number of colors among
//! permanents you control. Then you gain life equal to the number of creatures
//! you control."
//!
//! # GAP: ColorCountAmongPermanents — no script helper for counting distinct
//! colors among permanents you control. Token creation count is inexpressible;
//! Vec::new() returned for that part. Life gain uses script::count_matching.
//! GAP: Vivid keyword marker — not in the supported keyword list; omitted.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elemental Spectacle");
    let _elemental = reg.interner_mut().intern("Elemental");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Vivid — Create a number of 5/5 red and green Elemental creature tokens equal to the number of colors among permanents you control. Then you gain life equal to the number of creatures you control.".into(),
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
    // GAP: ColorCountAmongPermanents — no script helper for counting distinct colors
    // among permanents you control; token creation is omitted.
    let creature_count = script::count_matching(state, &ObjectFilter::creature(), entry.controller);
    let elemental = reg.interner().lookup("Elemental").expect("Elemental interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let _token = TokenDefinition {
        name: elemental,
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::GainLife { player: entry.controller, amount: creature_count },
    ]
}
