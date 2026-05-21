//! Desert's Due — `{1}{B}` instant. "Target creature gets -2/-2
//! until end of turn. It gets an additional -1/-1 until end of turn
//! for each Desert you control." Compose: -2 base plus -N from Desert
//! count.

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
    let name = reg.interner_mut().intern("Desert's Due");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets -2/-2 until end of turn. It gets an additional -1/-1 until end of turn for each Desert you control.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let desert_filter = script::subtype_filter(reg, "Desert").controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &desert_filter, entry.controller) as i32;
    let total = -(2 + n);
    vec![Effect::Pump { target: *id, power: total, toughness: total, duration: Duration::EndOfTurn, keywords: vec![] }]
}
