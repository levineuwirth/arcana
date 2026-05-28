//! Reidane, God of the Worthy // Valkmira, Protector's Shield
//!
//! Front: Legendary Creature — God {2}{W} 2/3 (white)
//!   Flying, vigilance
//!   Snow lands your opponents control enter tapped.
//!   Noncreature spells your opponents cast with mana value 4 or greater cost {2} more to cast.
//! Back: Legendary Artifact
//!   If a source an opponent controls would deal damage to you or a permanent you control, prevent 1 of that damage.
//!   Whenever you or another permanent you control becomes the target of a spell or ability an opponent controls, counter that spell or ability unless its controller pays {1}.
//! GAP: MDFC back face not modeled (mechanic deferred)
//! GAP: "Snow lands opponents control enter tapped" replacement effect not modeled
//! GAP: Cost-increase static ability not modeled
//! GAP: Damage prevention replacement effect not modeled
//! GAP: Counter-spell-unless-pay trigger not modeled

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reidane, God of the Worthy");
    let back_name = reg.interner_mut().intern("Valkmira, Protector's Shield");
    let god = reg.interner_mut().intern("God");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(god);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    let back_chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ARTIFACT.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
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
