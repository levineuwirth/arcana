//! Toils of Night and Day — `{2}{U}` instant — Arcane. "You may tap or
//! untap target permanent, then you may tap or untap another target
//! permanent."

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
    let name = reg.interner_mut().intern("Toils of Night and Day");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let perm_req = || TargetRequirement {
        filter: TargetFilter::Permanent(ObjectFilter::permanent()),
        count: TargetCount::Exactly(1),
        controller: None,
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You may tap or untap target permanent, then you may tap or untap another target permanent.".into(),
            target_requirements: vec![perm_req(), perm_req()],
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
    // "tap or untap" is a player choice not modeled; default to Tap of
    // each targeted permanent.
    entry
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::Tap { target: *id }),
            _ => None,
        })
        .collect()
}
