//! Angel of Invention — `{3}{W}{W}` 2/1 Angel with Flying, Vigilance,
//! Lifelink.
//!
//! Fabricate 2 — `KeywordAbility` has no `Fabricate` variant in this
//! surface, so it isn't emitted; the ETB "put two +1/+1 counters on it
//! or create two Servo tokens" choice is GAP'd.
//!
//! "Other creatures you control get +1/+1." — GAP: a pure static anthem
//! is not a triggered/activated ability and has no expressible primitive
//! here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angel of Invention");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };

    // GAP: Fabricate 2 ETB choice — no Fabricate keyword / ETB-choice
    // primitive in this surface.
    // GAP: static "Other creatures you control get +1/+1" anthem.
    reg.register(CardDefinition::new(name, chars))
}
