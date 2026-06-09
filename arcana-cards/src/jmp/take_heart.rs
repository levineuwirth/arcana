//! Take Heart — `{W}` instant. "Target creature gets +2/+2 until end
//! of turn. You gain 1 life for each attacking creature you control."
//!
//! The +2/+2 is a plain `Effect::Pump`; the life gain counts attacking
//! creatures you control via `ObjectFilter::attacking_only()`.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Take Heart");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets +2/+2 until end of turn. You gain 1 life for each attacking creature you control.".into(),
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
    let n = script::count_matching(
        state,
        &ObjectFilter::creature()
            .attacking_only()
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let mut effects = vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    if n > 0 {
        effects.push(Effect::GainLife {
            player: entry.controller,
            amount: n,
        });
    }
    effects
}
