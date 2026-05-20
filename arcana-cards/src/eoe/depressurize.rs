//! Depressurize — `{1}{B}` instant. "Target creature gets -3/-0 until
//! end of turn. Then if that creature's power is 0 or less, destroy
//! it."
//!
//! The dynamic check on resulting power runs at resolution time via
//! script::power_of (post-pump value is current power - 3).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Depressurize");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets -3/-0 until end of turn. Then if that creature's power is 0 or less, destroy it.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let mut effects = vec![Effect::Pump {
        target: *id,
        power: -3,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    let post = script::power_of(state, *id) - 3;
    if post <= 0 {
        effects.push(Effect::DestroyPermanent { target: *id });
    }
    effects
}
