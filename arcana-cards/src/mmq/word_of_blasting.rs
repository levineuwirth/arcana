//! Word of Blasting — `{1}{R}` instant. "Destroy target Wall. It can't be
//! regenerated. Word of Blasting deals damage equal to that Wall's mana
//! value to the Wall's controller."
//!
//! GAP: target subtype "Wall" must be a register-time filter, but
//! `script::subtype_filter` requires a runtime `&CardRegistry`; the
//! catalog has no register-time subtype constructor. Damage equal to a
//! target's CMC and a can't-be-regenerated rider also have no Effect
//! variants. Falling back to plain creature target + destroy.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Word of Blasting");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target Wall. It can't be regenerated. Word of Blasting deals damage equal to that Wall's mana value to the Wall's controller.".into(),
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
    // GAP: no CMC-of-target / controller-of-target accessor for the
    // damage rider; no can't-be-regenerated effect. Destroy only.
    vec![Effect::DestroyPermanent { target: *id }]
}
