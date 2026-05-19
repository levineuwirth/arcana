//! Oversimplify — `{3}{G}{U}` sorcery. "Exile all creatures. Each player
//! creates a 0/0 green and blue Fractal creature token and puts a number of
//! +1/+1 counters on it equal to the total power of creatures they controlled
//! that were exiled this way."
//!
//! # GAP: tracking per-player total power of exiled creatures for counter placement
//! The engine has no way to sum power of exiled permanents and place that many
//! counters on the created token.  Best-effort: exile all creatures and create
//! a 0/0 Fractal token for the controller only (opponent token creation and
//! dynamic counter counts are dropped).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oversimplify");
    let _fractal = reg.interner_mut().intern("Fractal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile all creatures. Each player creates a 0/0 green and blue Fractal creature token and puts a number of +1/+1 counters on it equal to the total power of creatures they controlled that were exiled this way.".into(),
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
    let all_creatures = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let fractal = reg.interner().lookup("Fractal")
        .expect("Fractal interned during register()");
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
    let mut effects = Vec::new();
    if !all_creatures.is_empty() {
        effects.push(Effect::ForEach {
            targets: all_creatures,
            effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
        });
    }
    // GAP: per-player summed-power counter placement; opponent token creation
    effects.push(Effect::CreateToken { controller: entry.controller, token });
    effects
}
