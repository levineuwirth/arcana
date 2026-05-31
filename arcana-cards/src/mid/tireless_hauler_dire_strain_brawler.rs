//! Tireless Hauler // Dire-Strain Brawler — {4}{G} Creature — Human Werewolf (4/5)
//! transforming DFC (werewolf). Front: Vigilance, Daybound. Back (Dire-Strain Brawler):
//! Creature — Werewolf (4/5), Vigilance, Nightbound.
//!
//! GAP: Daybound / Nightbound are not modeled keyword abilities in this engine
//! (no KeywordAbility::Daybound/Nightbound variant). The day/night transform
//! cadence those keywords drive is therefore not authored here; both faces are
//! emitted with their printed Vigilance and P/T, and the day/night-driven
//! transform is left to future engine work.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tireless Hauler");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Dire-Strain Brawler");
    let back = arcana_core::registry::CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Vigilance],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
