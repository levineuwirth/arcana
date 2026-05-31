//! Swarmyard Massacre — `{3}{B}{B}` sorcery. "Create two 1/1 green
//! Squirrel creature tokens. Then each creature that isn't an Insect,
//! Rat, Spider, or Squirrel gets -1/-1 until end of turn for each
//! creature you control that's an Insect, Rat, Spider, or Squirrel."
//!
//! The two Squirrel tokens are created faithfully. The mass -X/-X is
//! GAP-ed: it must target "each creature that ISN'T an Insect, Rat,
//! Spider, or Squirrel", but `ObjectFilter` exposes only
//! `.with_subtypes_any` (positive subtype match) — there is no
//! negated-subtype filter to select the non-swarm creatures, so the
//! affected set cannot be enumerated.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Swarmyard Massacre");
    let _squirrel = reg.interner_mut().intern("Squirrel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create two 1/1 green Squirrel creature tokens. Then each creature that isn't an Insect, Rat, Spider, or Squirrel gets -1/-1 until end of turn for each creature you control that's an Insect, Rat, Spider, or Squirrel.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let squirrel = reg.interner().lookup("Squirrel")
        .expect("Squirrel interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    let token = TokenDefinition {
        name: squirrel,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "each creature that isn't an Insect, Rat, Spider, or Squirrel
    // gets -1/-1 ... for each creature you control that's an Insect, Rat,
    // Spider, or Squirrel" — selecting the affected set requires a
    // negated-subtype filter (creatures that are NOT those subtypes),
    // which ObjectFilter does not provide (only positive
    // `.with_subtypes_any`). The mass pump cannot be enumerated.
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
