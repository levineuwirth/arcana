//! Hogaak, Arisen Necropolis — `{5}{B/G}{B/G}` 8/8 Legendary Avatar.
//! "You can't spend mana to cast this spell. Convoke, delve. You may cast
//! this card from your graveyard. Trample."
//!
//! Only Trample is expressible from the keyword line. Convoke and delve are
//! not in the supported keyword surface, and the "can't spend mana" /
//! "cast from your graveyard" alternative-casting statics are not
//! expressible — all recorded as GAPs. Bones plus Trample are emitted.
//!
//! GAP: Convoke (alternative-cost cast mechanic, not in the keyword surface).
//! GAP: delve (alternative-cost cast mechanic, not in the keyword surface).
//! GAP: static — "You can't spend mana to cast this spell."
//! GAP: static — "You may cast this card from your graveyard."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hogaak, Arisen Necropolis");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B/G}{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
