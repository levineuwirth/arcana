//! Malanthrope — `{1}{G}{U}` 2/2 Tyranid with Flying.
//! "Scavenge the Dead — When this creature enters, exile target player's
//! graveyard. Put a +1/+1 counter on this creature for each creature card
//! exiled this way." The ETB is a GAP (no exile-entire-graveyard effect).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Malanthrope");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: ETB "exile target player's graveyard, then +1/+1 counter per
    // creature card exiled this way" — no effect exiles an entire graveyard,
    // and the counter count derives from that exile event. Omitted.
    reg.register(CardDefinition::new(name, chars))
}
