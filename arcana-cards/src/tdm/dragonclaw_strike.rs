//! Dragonclaw Strike — `{2/G}{2/U}{2/R}` sorcery.
//! "Double the power and toughness of target creature you control until end
//! of turn. Then it fights up to one target creature an opponent controls."
//!
//! Note: doubling P/T is modeled as Pump +X/+X where X = current power/toughness
//! via script helpers.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dragonclaw Strike");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2/G}{2/U}{2/R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Double the power and toughness of target creature you control until end of turn. Then it fights up to one target creature an opponent controls.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement {
                        filter: arcana_core::targets::TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
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
    let Some(t0) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(a) = t0 else { return Vec::new(); };
    let power = script::power_of(state, *a);
    let toughness = script::toughness_of(state, *a);
    let mut effects = vec![Effect::Pump {
        target: *a,
        power,
        toughness,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    if let Some(t1) = entry.targets.targets.get(1) {
        if let TargetChoice::Object(b) = t1 {
            effects.push(Effect::Fight { a: *a, b: *b });
        }
    }
    effects
}
