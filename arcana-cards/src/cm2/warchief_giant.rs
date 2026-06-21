//! Warchief Giant — `{3}{R}{R}` 5/3 Giant Warrior.
//! "Haste"
//! "Myriad (Whenever this creature attacks, for each opponent other than defending
//!  player, you may create a token copy that's tapped and attacking that player or
//!  a planeswalker they control. Exile the tokens at end of combat.)"
//!
//! Haste is a keyword. Myriad is not in the usable KeywordAbility surface and its
//! attack behavior — minting per-opponent tapped-and-attacking token copies of
//! this creature, exiled at end of combat — has no expressible Effect path
//! (CopyPermanent does not produce attacking, end-of-combat-exiled copies fanned
//! out per opponent), so it is a GAP.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Warchief Giant");
    let giant = reg.interner_mut().intern("Giant");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keyword — Myriad not in the usable KeywordAbility surface; its
        // per-opponent tapped-attacking token-copy attack behavior is unexpressible.
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
