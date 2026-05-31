//! Excise the Imperfect — `{1}{W}{W}` instant, "Exile target nonland
//! permanent. Its controller incubates X, where X is its mana value."
//!
//! The exile is expressed faithfully. The "incubates X" rider mints an
//! Incubator token with X +1/+1 counters where X is the exiled
//! permanent's mana value — but the catalog's `CommodityToken::Incubator`
//! mints only a bare token (no per-counter X), and the exiled object's
//! mana value is no longer queryable once it has left the battlefield.
//! So the incubate rider is GAPped.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Excise the Imperfect");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target nonland permanent. Its controller incubates X, where X is its mana value.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

use arcana_core::mana::ManaCost;

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "Its controller incubates X (X = its mana value)" — Incubate with
    // a dynamic counter count is not expressible (CommodityToken::Incubator
    // mints a bare token only, and the exiled object's mana value is not
    // queryable post-exile). Only the exile is emitted.
    vec![Effect::ExilePermanent { target: *id }]
}
