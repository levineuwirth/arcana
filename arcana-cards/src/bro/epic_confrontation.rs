//! Epic Confrontation — `{1}{G}` sorcery, "Target creature you control
//! gets +1/+2 until end of turn. It fights target creature you don't
//! control."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Epic Confrontation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +1/+2 until end of turn. It fights target creature you don't control.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_creature(),
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
    if entry.targets.targets.len() < 2 { return Vec::new(); }
    let id_a = match &entry.targets.targets[0] {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };
    let id_b = match &entry.targets.targets[1] {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };
    vec![
        Effect::Pump {
            target: id_a,
            power: 1,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::Fight { a: id_a, b: id_b },
    ]
}
