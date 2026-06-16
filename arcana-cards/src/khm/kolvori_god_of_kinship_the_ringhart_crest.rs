//! Kolvori, God of Kinship // The Ringhart Crest
//!
//! Front: Legendary Creature — God {2}{G}{G} 2/4 (green)
//!   As long as you control three or more legendary creatures, Kolvori gets +4/+2 and has vigilance.
//!   {1}{G}, {T}: Look at the top six cards. May reveal a legendary creature card and put it to hand. Rest on bottom in random order.
//! Back: Legendary Artifact (land-like permanent) — GAP: MDFC back face not modeled (mechanic deferred)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kolvori, God of Kinship");
    let back_name = reg.interner_mut().intern("The Ringhart Crest");
    let god = reg.interner_mut().intern("God");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(god);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    let back_chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ARTIFACT.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(CardFace {
                name: back_name,
                characteristics: back_chars,
                spell_ability: None,
            }),
    )
}
