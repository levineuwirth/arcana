//! Accelerated Mutation — `{3}{G}{G}` instant. "Target creature gets
//! +X/+X until end of turn, where X is the greatest mana value among
//! permanents you control." Dynamic X computed via
//! `script::max_cmc_of` over all permanents the caster controls.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Accelerated Mutation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets +X/+X until end of turn, where X is the greatest mana value among permanents you control.".into(),
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
    let x = script::max_cmc_of(
        state,
        &ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    vec![Effect::Pump {
        target: *id,
        power: x as i32,
        toughness: x as i32,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
