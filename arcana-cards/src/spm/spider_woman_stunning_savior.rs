//! Spider-Woman, Stunning Savior — `{1}{W/U}` 2/2 Legendary Spider Human Hero with Flying.
//!
//! * Flying (keyword).
//! * "Venom Blast — Artifacts and creatures your opponents control enter tapped."
//!   — opponent-side enters-tapped static replacement, not expressible, GAP.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spider-Woman, Stunning Savior");
    let spider = reg.interner_mut().intern("Spider");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(human);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W/U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Artifacts and creatures your opponents control enter tapped" — opponent-
    // side enters-tapped static replacement effect, not expressible.
    reg.register(CardDefinition::new(name, chars))
}
