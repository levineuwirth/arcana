//! Burrog Barrage — `{1}{G}` instant, "Target creature you control gets
//! +1/+0 until end of turn if you've cast another instant or sorcery spell
//! this turn. Then it deals damage equal to its power to up to one target
//! creature an opponent controls."
//!
//! GAP: conditional +1/+0 based on instant/sorcery-cast-this-turn history
//! and power-scaled damage from one creature to another not fully
//! expressible; expressed as Fight between the two targets.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Burrog Barrage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +1/+0 until end of turn if you've cast another instant or sorcery spell this turn. Then it deals damage equal to its power to up to one target creature an opponent controls.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement {
                        filter: TargetFilter::Creature,
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
    // GAP: conditional +1/+0 from instant/sorcery history; power-scaled damage
    if entry.targets.targets.len() < 2 { return Vec::new(); }
    let id_a = match &entry.targets.targets[0] {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };
    let id_b = match &entry.targets.targets[1] {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };
    vec![Effect::Fight { a: id_a, b: id_b }]
}
