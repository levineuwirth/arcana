//! Uvilda, Dean of Perfection // Nassari, Dean of Expression
//!
//! Front: Legendary Creature — Djinn Wizard {2}{U} 2/2 (blue)
//!   {T}: Exile an instant or sorcery card from your hand and put three hone counters on it (complex deferred mechanic).
//! Back: Legendary Creature — Efreet Shaman
//!   At the beginning of your upkeep, exile the top card of each opponent's library. Until end of turn, you may cast those cards.
//!   Whenever you cast a spell from exile, put a +1/+1 counter on Nassari.
//! GAP: MDFC back face not modeled (mechanic deferred)
//! GAP: Hone counter activated ability not modeled
//! GAP: "Exile top card of each opponent's library, may cast them" not in Effect catalog
//! GAP: "Cast from exile" trigger not modeled

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Uvilda, Dean of Perfection");
    let back_name = reg.interner_mut().intern("Nassari, Dean of Expression");
    let djinn = reg.interner_mut().intern("Djinn");
    let wizard = reg.interner_mut().intern("Wizard");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(djinn);
    subtypes.insert(wizard);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    let efreet = reg.interner_mut().intern("Efreet");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut back_subtypes = SubtypeSet::new();
    back_subtypes.insert(efreet);
    back_subtypes.insert(shaman);

    let back_chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes: back_subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_mdfc_back(CardFace {
            name: back_name,
            characteristics: back_chars,
            spell_ability: None,
        }),
    )
}
