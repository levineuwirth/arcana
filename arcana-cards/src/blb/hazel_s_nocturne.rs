//! Hazel's Nocturne — `{3}{B}` instant. "Return up to two target creature
//! cards from your graveyard to your hand. Each opponent loses 2 life and you
//! gain 2 life."
//!
//! GAP: "each opponent loses 2 life" requires iterating over all opponents;
//! the catalog has no loop-over-opponents primitive. Only the first two effects
//! (return creatures to hand for up to 2 targets) and self-gain-life are
//! expressed. The "each opponent loses life" clause is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hazel's Nocturne");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to two target creature cards from your graveyard to your hand. Each opponent loses 2 life and you gain 2 life.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::creature() },
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::ReturnFromGraveyardToHand { target: *id })
        } else {
            None
        }
    }).collect();
    // GAP: "each opponent loses 2 life" — no loop-over-opponents primitive
    effects.push(Effect::GainLife { player: entry.controller, amount: 2 });
    effects
}
