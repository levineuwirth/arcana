//! Questionable Cuisine — `{3}{B}` sorcery. "Return up to two target
//! creature cards from your graveyard to your hand. Create a Food token
//! for each trash can you can see from your seat."
//!
//! The graveyard recursion is fully expressible: two `UpTo(1)` graveyard
//! creature-card targets, each returned to hand with
//! `Effect::ReturnFromGraveyardToHand`. The Food count is gated on a
//! real-world, non-deterministic quantity ("trash cans you can see from
//! your seat") that no `script::*` helper can compute — so the token
//! creation is GAP-ped rather than hardcoded to a wrong literal.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Questionable Cuisine");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return up to two target creature cards from your graveyard to your hand. Create a Food token for each trash can you can see from your seat.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
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
    let mut effects = Vec::new();
    for target in &entry.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
        }
    }
    // GAP: "Create a Food token for each trash can you can see from your
    // seat" — the count is a real-world, non-deterministic quantity that
    // no script:: helper can compute, so the Food creation is omitted
    // rather than emitting a wrong literal count.
    effects
}
