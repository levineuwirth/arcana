//! Violent Ultimatum — `{B}{B}{R}{R}{R}{G}{G}` sorcery. "Destroy three
//! target permanents."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Violent Ultimatum");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{R}{R}{R}{G}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy three target permanents.".into(),
                target_requirements: vec![
                    TargetRequirement::any_target(),
                    TargetRequirement::any_target(),
                    TargetRequirement::any_target(),
                ],
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
    entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::DestroyPermanent { target: *id })
        } else {
            None
        }
    }).collect()
}
