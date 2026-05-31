//! Chelonian Tackle — `{2}{G}` sorcery, "Target creature you control
//! gets +0/+10 until end of turn. Then it fights up to one target
//! creature an opponent controls."
//!
//! Two targets: a creature you control (which gets +0/+10) and up to one
//! creature an opponent controls that it then fights.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chelonian Tackle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control gets +0/+10 until end of turn. Then it fights up to one target creature an opponent controls.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
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
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut targets = entry.targets.targets.iter();
    let Some(TargetChoice::Object(mine)) = targets.next() else {
        return Vec::new();
    };
    let mut effects = vec![Effect::Pump {
        target: *mine,
        power: 0,
        toughness: 10,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    if let Some(TargetChoice::Object(foe)) = targets.next() {
        effects.push(Effect::Fight { a: *mine, b: *foe });
    }
    effects
}
