//! Spectacle Mage — `{1}{U}{R}` 2/2 blue-red Bird Shaman with Flying.
//! "Instant and sorcery spells you cast with mana value 5 or greater
//! cost {1} less to cast." — GAP: this is a static cost-reduction
//! continuous ability (no trigger, no activation cost) and there is no
//! demonstrated primitive for spell cost reduction, so it is omitted.
//! Only the Flying keyword is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spectacle Mage");
    let bird = reg.interner_mut().intern("Bird");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP: "Instant and sorcery spells you cast with mana value 5 or
    // greater cost {1} less to cast." — static spell cost reduction, no
    // demonstrated primitive.
    reg.register(CardDefinition::new(name, chars))
}
