//! Valor — `{3}{W}` 2/2 white Incarnation with First strike.
//!
//! "As long as this card is in your graveyard and you control a Plains,
//! creatures you control have first strike." is a graveyard-resident
//! static continuous ability granting a keyword to your whole board.
//! The MultiAbilityCreature shape has no static-from-graveyard
//! continuous-grant primitive (it composes triggered / activated
//! abilities and the printed keyword line only), so the static is GAP'd.
//! The on-battlefield First strike keyword is emitted faithfully.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Valor");
    let incarnation = reg.interner_mut().intern("Incarnation");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(incarnation);

    // GAP: "As long as this card is in your graveyard and you control a
    // Plains, creatures you control have first strike." — a static
    // continuous keyword-grant ability resident in the graveyard; not
    // expressible in the MultiAbilityCreature shape (no static
    // board-wide grant / graveyard-zone static primitive here).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
