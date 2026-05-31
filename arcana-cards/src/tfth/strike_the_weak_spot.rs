//! Strike the Weak Spot — Sorcery. "Destroy target Head. If that Head
//! was elite, the Hydra takes an extra turn after this one."
//!
//! The destroy half is expressible as a destroy of a target Head
//! (filtered by the "Head" subtype). The "if that Head was elite, the
//! Hydra takes an extra turn" rider relies on the Arena boss-battle
//! "elite"/extra-turn mechanic, which is not modeled by the engine.

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
    let name = reg.interner_mut().intern("Strike the Weak Spot");
    let head = reg.interner_mut().intern("Head");
    let chars = Characteristics {
        name,
        colors: ColorSet::new(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let filter = ObjectFilter::creature().with_subtypes_any(vec![head]);
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target Head. If that Head was elite, the Hydra takes an extra turn after this one.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(filter),
                count: TargetCount::Exactly(1),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "if that Head was elite, the Hydra takes an extra turn" — the
    // Arena boss-battle elite/extra-turn mechanic is not modeled.
    vec![Effect::DestroyPermanent { target: *id }]
}
