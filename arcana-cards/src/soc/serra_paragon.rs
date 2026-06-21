//! Serra Paragon — `{2}{W}{W}` 3/4 Angel with Flying.
//!
//! Oracle:
//! * Flying — keyword line.
//! * Once during each of your turns, you may play a land from your
//!   graveyard or cast a permanent spell with mana value 3 or less from
//!   your graveyard. If you do, it gains "When this permanent is put
//!   into a graveyard from the battlefield, exile it and you gain 2
//!   life." — GAP: a static once-per-turn play/cast-from-graveyard
//!   permission with a granted death trigger; not a triggered or
//!   activated ability and not expressible with the available
//!   primitives.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serra Paragon");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
