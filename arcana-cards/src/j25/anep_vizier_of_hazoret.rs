//! Anep, Vizier of Hazoret — `{2}{R}` 4/2 Legendary Creature — Jackal Warrior
//! with Trample.
//!
//! Oracle:
//! * Trample
//! * You may exert Anep as it attacks. When you do, exile the top two cards of
//!   your library. Until the end of your next turn, you may play those cards.
//!
//! Trample is a base keyword. The exert ability is GAP'd: Exert is not in the
//! usable `KeywordAbility` surface and there is no "you may exert as it attacks"
//! / "when you exert this" trigger condition. The exile-and-play payoff
//! (which on its own would be `Effect::ImpulseExile`) is gated behind that
//! unmodeled exert action and so cannot fire.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anep, Vizier of Hazoret");
    let jackal = reg.interner_mut().intern("Jackal");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jackal);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Exert — not in the usable KeywordAbility surface.
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "You may exert Anep as it attacks. When you do, exile the top two
    // cards ...; play them until end of your next turn" — no exert trigger
    // condition, so the (otherwise ImpulseExile-able) payoff can't fire.
    reg.register(CardDefinition::new(name, chars))
}
