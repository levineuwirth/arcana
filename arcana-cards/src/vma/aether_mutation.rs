//! Aether Mutation — `{3}{G}{U}` sorcery. "Return target creature to
//! its owner's hand. Create X 1/1 green Saproling creature tokens,
//! where X is that creature's mana value."
//!
//! The bounce half is expressed with `Effect::ReturnToHand`. The token
//! half is dynamic — X equals the bounced creature's mana value — and
//! there is no `script::` helper for a target permanent's mana value
//! (and after the bounce the creature has left the battlefield), so the
//! Saproling creation is recorded as a GAP rather than a wrong
//! fixed-count token effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aether Mutation");
    let _saproling = reg.interner_mut().intern("Saproling");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return target creature to its owner's hand. Create X 1/1 green Saproling creature tokens, where X is that creature's mana value.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
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
    // GAP: no script helper for a target permanent's mana value, so the
    // "create X 1/1 green Saproling tokens, where X is that creature's
    // mana value" rider cannot be computed and is omitted.
    vec![Effect::ReturnToHand { target: *id }]
}
