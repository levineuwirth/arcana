//! Bird Admirer // Wing Shredder — `{2}{G}` Human Archer Werewolf creature 1/4
//! with Reach and Daybound/Nightbound (transform layout).
//!
//! # Rules text (front face — Bird Admirer)
//! Reach
//! Daybound (If a player casts no spells during their own turn, it becomes
//! night next turn.)
//!
//! # Rules text (back face — Wing Shredder)
//! Reach
//! Nightbound (If a player casts at least two spells during their own turn,
//! it becomes day next turn.)
//!
//! # GAPs
//! - Daybound/Nightbound day/night cycle triggers are not modeled — the
//!   engine does not track the day/night state machine. Transform is
//!   registered via the shape skeleton but the precise "no spells cast last
//!   turn" / "two spells cast this turn" trigger conditions are deferred.
//!   // GAP: Daybound/Nightbound transform trigger conditions not modeled
//!   (day/night cycle unimplemented in engine).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bird Admirer");
    let human_sub = reg.interner_mut().intern("Human");
    let archer_sub = reg.interner_mut().intern("Archer");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(archer_sub);
    subtypes.0.insert(werewolf_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid creature cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // Back face — Wing Shredder (Werewolf)
    let back_name = reg.interner_mut().intern("Wing Shredder");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Reach],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: Daybound/Nightbound transform trigger conditions not modeled
    // (day/night cycle unimplemented in engine). No transform trigger wired.
    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
