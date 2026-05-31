//! Death Mutation — `{6}{B}{G}` sorcery. "Destroy target nonblack
//! creature. It can't be regenerated. Create X 1/1 green Saproling
//! creature tokens, where X is that creature's mana value."
//!
//! The destroy is expressed faithfully. "It can't be regenerated" has
//! no dedicated primitive (regeneration-shield suppression is engine
//! debt) and is noted as a GAP. The token clause is GAP-ped: X is the
//! destroyed creature's mana value, and there is no `script::` helper
//! that returns an object's mana value (only power/toughness), so the
//! dynamic token count cannot be computed — emitting a fixed count
//! would be materially wrong.

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Death Mutation");
    let _saproling = reg.interner_mut().intern("Saproling");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target nonblack creature. It can't be regenerated. Create X 1/1 green Saproling creature tokens, where X is that creature's mana value.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().without_colors(ColorSet::black()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "It can't be regenerated" has no dedicated primitive.
    // GAP: "Create X Saproling tokens where X is that creature's mana value" —
    // no script helper returns an object's mana value, so the dynamic token
    // count is not expressible. Only the destroy is emitted.
    vec![Effect::DestroyPermanent { target: *id }]
}
