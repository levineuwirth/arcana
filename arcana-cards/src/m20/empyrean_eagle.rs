//! Empyrean Eagle — `{1}{W}{U}` Creature — Bird Spirit, 2/3, Flying.
//! "Other creatures you control with flying get +1/+1." — a printed static
//! `ContinuousEffect::filtered_pump` (creatures you control with Flying)
//! that the engine installs as the Eagle enters (A1.5br).
//! A1.5bq excludes this source and reads flying after layer 6.

use arcana_core::effects::KeywordAbility;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Empyrean Eagle");
    let bird = reg.interner_mut().intern("Bird");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    let filter = ObjectFilter::creature()
        .excluding_source()
        .controlled_by(ControllerConstraint::You)
        .with_keyword(KeywordAbility::Flying);
    reg.register(
        CardDefinition::new(name, chars).with_static(vec![
            ContinuousEffect::filtered_pump(
                NULL_OBJECT_ID,
                filter,
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        ]),
    )
}
