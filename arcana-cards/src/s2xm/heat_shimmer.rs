//! Heat Shimmer — `{2}{R}` sorcery. "Create a token that's a copy of
//! target creature, except it has haste and \"At the beginning of the
//! end step, exile this token.\""
//!
//! The core copy is expressed via `Effect::CopyPermanent`, which mints a
//! token copy of the target creature. The "except it has haste and the
//! end-step exile rider" cannot be attached: the new token's object id is
//! not knowable from a separate effect (CopyPermanent produces the token
//! internally), so neither the granted Haste nor the delayed self-exile
//! can reference it. See GAP below.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heat Shimmer");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a token that's a copy of target creature, except it has haste and \"At the beginning of the end step, exile this token.\"".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: the copy token's "haste" grant and the "at the beginning of the
    // end step, exile this token" delayed-exile rider cannot be attached —
    // CopyPermanent mints the token internally, so its id is not available
    // to a follow-up GrantKeyword / DelayedAction in this resolver.
    vec![Effect::CopyPermanent { target: *id }]
}
