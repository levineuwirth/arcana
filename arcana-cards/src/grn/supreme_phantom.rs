//! Supreme Phantom — `{1}{U}` Spirit, 1/3, flying.
//! Other Spirits you control get +1/+1 (A1.5bq), a printed static the
//! engine installs as the Phantom enters (A1.5br).

use arcana_core::effects::KeywordAbility;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Supreme Phantom");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars).with_static(vec![
        ContinuousEffect::filtered_pump(
            NULL_OBJECT_ID,
            ObjectFilter::creature().with_subtype_sym(spirit)
                .controlled_by(ControllerConstraint::You).excluding_source(),
            1, 1, Duration::WhileSourceOnBattlefield,
        ),
    ]))
}
