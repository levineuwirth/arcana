//! Eye of Duskmantle — `{5}{B}{B}` 3/8 Eye.
//!
//! Oracle:
//! * Flying, lifelink.
//! * You may play lands and cast spells from among cards in your graveyard
//!   you've surveilled this turn. If you cast a spell this way, you pay life
//!   equal to its mana value rather than paying its mana cost.
//!
//! The keyword line is fully modeled. The play-from-graveyard permission with
//! the life-instead-of-mana alternate cost has no primitive and is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eye of Duskmantle");
    let eye = reg.interner_mut().intern("Eye");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eye);

    // GAP: static — "play lands / cast spells from among cards you've surveilled
    // this turn in your graveyard; pay life equal to mana value instead"
    // (graveyard-play permission + alternate life cost; no primitive).

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
