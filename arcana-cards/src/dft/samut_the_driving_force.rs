//! Samut, the Driving Force — `{3}{R}{G}{W}` 4/5 Legendary Human Warrior
//! Cleric.
//!
//! Oracle:
//! * First strike, vigilance, haste (keyword line).
//! * "Start your engines!" — the speed mechanic; not in the usable keyword
//!   surface and its speed-tracking is unmodeled. GAP'd.
//! * "Other creatures you control get +X/+0, where X is your speed." — a
//!   static anthem keyed on speed; not a triggered/activated ability and
//!   speed is unmodeled. GAP'd.
//! * "Noncreature spells you cast cost {X} less to cast, where X is your
//!   speed." — a static cost-reduction; not a triggered/activated ability.
//!   GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Samut, the Driving Force");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::FirstStrike,
            KeywordAbility::Vigilance,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    // GAP: "Start your engines!" — speed mechanic not in the usable keyword
    // surface; speed is unmodeled.
    // GAP: static anthem "Other creatures you control get +X/+0, where X is
    // your speed" — a continuous static keyed on speed.
    // GAP: static cost-reduction "Noncreature spells you cast cost {X} less,
    // where X is your speed" — a continuous cost-altering static.
    reg.register(CardDefinition::new(name, chars))
}
