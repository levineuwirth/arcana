//! Burly Breaker // Dire-Strain Demolisher — {3}{G}{G} Human Werewolf 6/5
//!
//! Front face: Ward {1}, Daybound
//! Back face: Dire-Strain Demolisher — Werewolf, Ward {3}, Nightbound
//!
//! GAP: Daybound / Nightbound — the day/night cycle and the precise "no spells
//! cast last turn" / "two spells cast this turn" werewolf transform conditions
//! are not modeled. Transform triggers are omitted; both Ward keywords are
//! authored correctly. The back face is registered with Ward {3}.
//! GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Burly Breaker");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid cost"))],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Dire-Strain Demolisher");
    let werewolf_back_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(8)),
            toughness: Some(PtValue::Fixed(7)),
            keywords: vec![KeywordAbility::Ward(ManaCost::parse("{3}").expect("valid cost"))],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
