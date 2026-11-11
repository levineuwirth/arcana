//! Goblin Warchief — `{1}{R}{R}` 2/2 Goblin Warrior.
//! "Goblin spells you cast cost {1} less to cast. Goblins you control
//! have haste." Two printed static abilities, a cost reduction and a
//! keyword grant, which the engine installs as the Warchief enters
//! (A1.5br).

use arcana_core::effects::KeywordAbility;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Warchief");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Goblin spells you cast cost {1} less to cast."
            .with_static(vec![ContinuousEffect::spell_cost_modifier(
                NULL_OBJECT_ID,
                ObjectFilter::default().with_subtype_sym(goblin),
                ControllerConstraint::You,
                -1,
                Duration::WhileSourceOnBattlefield,
            )])
            // "Goblins you control have haste." It names no "other", so
            // the Warchief has haste itself.
            .with_static(vec![ContinuousEffect::filtered_keyword(
                NULL_OBJECT_ID,
                ObjectFilter::default().with_subtype_sym(goblin)
                    .controlled_by(ControllerConstraint::You),
                KeywordAbility::Haste,
                Duration::WhileSourceOnBattlefield,
            )]),
    )
}
