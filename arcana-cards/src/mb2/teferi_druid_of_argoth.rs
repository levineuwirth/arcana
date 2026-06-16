//! Teferi, Druid of Argoth — `{2}{G}{G}{G}` 3/4 Legendary Human Druid.
//! Flash.
//! Creature cards you own off the battlefield have flash (static — GAP).
//! Each opponent can cast spells only as a sorcery (static — GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teferi, Druid of Argoth");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    // GAP (static): "Creature cards you own that aren't on the battlefield have
    //   flash" — granting flash to cards in other zones is not expressible here.
    // GAP (static): "Each opponent can cast spells only any time they could cast
    //   a sorcery" — opponent casting-restriction static is not expressible.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
