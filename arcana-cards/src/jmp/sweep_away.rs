//! Sweep Away — `{2}{U}` instant. "Return target creature to its owner's
//! hand. If that creature is attacking, you may put it on top of its
//! owner's library instead."
//! Attacking status checked directly against `state.combat` at resolve.
//! GAP: the "you may ... instead" choice is auto-yes (always puts an
//! attacking creature on top of its owner's library).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sweep Away");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return target creature to its owner's hand. If that creature is attacking, you may put it on top of its owner's library instead.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // If the creature is attacking, put it on top of its owner's library
    // instead (the "you may" is auto-yes; see module GAP note).
    if state.combat.as_ref().is_some_and(|c| c.is_attacker(*id)) {
        return vec![Effect::PutOnTopOfLibrary { target: *id }];
    }
    vec![Effect::ReturnToHand { target: *id }]
}
