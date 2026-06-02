//! Rune Snag — `{1}{U}` instant. "Counter target spell unless its
//! controller pays {2} plus an additional {2} for each card named
//! Rune Snag in each graveyard."
//!
//! The tax is dynamic: {2} base plus {2} per Rune Snag in every
//! graveyard. We count those cards across all players' graveyards via
//! `script::graveyard_matching` with a name filter and build the
//! generic mana cost at resolution, then feed it to
//! `Effect::CounterUnlessPays`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rune Snag");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell unless its controller pays {2} plus an additional {2} for each card named Rune Snag in each graveyard.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(ObjectFilter::default()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let stack_id = match target {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };

    // {2} per Rune Snag card in each (every) graveyard.
    let nm = reg.interner().lookup("Rune Snag");
    let filter = ObjectFilter { name: nm, ..ObjectFilter::default() };
    let mut copies: u32 = 0;
    for p in script::all_players(state) {
        copies += script::graveyard_matching(state, &filter, p, entry.controller);
    }

    let generic = 2 + 2 * copies;
    let cost = ManaCost::parse(&format!("{{{}}}", generic)).expect("valid cost");

    vec![Effect::CounterUnlessPays { target: stack_id, cost }]
}
