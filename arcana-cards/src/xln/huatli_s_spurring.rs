//! Huatli's Spurring — `{R}` instant. "Target creature gets +2/+0
//! until end of turn. If you control a Huatli planeswalker, that
//! creature gets +4/+0 until end of turn instead."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Huatli's Spurring");
    let _huatli = reg.interner_mut().intern("Huatli");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets +2/+0 until end of turn. If you control a Huatli planeswalker, that creature gets +4/+0 until end of turn instead.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let huatli = script::subtype_filter(reg, "Huatli")
        .with_types(TypeLine::PLANESWALKER.into())
        .controlled_by(ControllerConstraint::You);
    let has_huatli = script::count_matching(state, &huatli, entry.controller) > 0;
    let power = if has_huatli { 4 } else { 2 };
    vec![Effect::Pump {
        target: *id,
        power,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
