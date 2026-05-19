//! Joust — `{1}{R}` sorcery. "Choose target creature you control and target
//! creature you don't control. The creature you control gets +2/+1 until end
//! of turn if it's a Knight. Then those creatures fight each other."
//!
//! GAP: conditional pump based on creature subtype (Knight) at resolution
//! is not expressible with ObjectFilter. The fight effect is modeled;
//! the Knight-conditional pump is applied unconditionally as a best effort.

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
    let name = reg.interner_mut().intern("Joust");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature you control and target creature you don't control. The creature you control gets +2/+1 until end of turn if it's a Knight. Then those creatures fight each other.".into(),
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
    let mut iter = entry.targets.targets.iter();
    let first = iter.next();
    let second = iter.next();
    let (Some(t1), Some(t2)) = (first, second) else { return Vec::new(); };
    let (TargetChoice::Object(id1), TargetChoice::Object(id2)) = (t1, t2) else { return Vec::new(); };
    // GAP: Knight-subtype check at resolution; pump applied unconditionally as best effort
    vec![
        Effect::Pump {
            target: *id1,
            power: 2,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::Fight { a: *id1, b: *id2 },
    ]
}
