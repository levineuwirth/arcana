//! Reborn Hero — `{2}{W}` 2/2 white Human Soldier.
//!
//! Vigilance.
//! Threshold — As long as there are seven or more cards in your
//! graveyard, this creature has "When this creature dies, you may pay
//! {W}{W}. If you do, return this card to the battlefield under your
//! control." (GAP — a graveyard-size-gated conditional grant of a
//! triggered ability is not expressible as a static here.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reborn Hero");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // Threshold is not in the usable keyword surface (it is a
        // graveyard-size static, not an evergreen keyword).
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: Threshold static — "as long as 7+ cards in your graveyard, this
    // has 'when it dies, you may pay {W}{W} to return it'." A graveyard-
    // size-gated continuous grant of a conditional death trigger cannot be
    // expressed as a static for this card class.

    reg.register(CardDefinition::new(name, chars))
}
