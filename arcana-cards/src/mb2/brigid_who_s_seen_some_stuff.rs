//! Brigid, Who's Seen Some Stuff — `{2}{W}{W}` 3/3 Legendary Kithkin Archer.
//!
//! # Rules text
//!
//! * Vigilance.
//! * Nimble — "This creature can't be blocked by creatures with power 3 or
//!   greater." (Static evasion ability.)
//! * "Kithkin you control have thoughtweft." (Static ability granting the
//!   thoughtweft keyword-sharing ability to a creature subtype you control.)
//!
//! Only the Vigilance keyword is expressible with the demonstrated API.
//! Nimble is a keyword the engine's `KeywordAbility` surface does not carry,
//! and the thoughtweft grant is a static continuous ability (no trigger, no
//! cost) — neither maps to a `KeywordAbility`, `TriggeredAbilityDef`, or
//! `ActivatedAbilityDef`, so both are GAP'd below.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brigid, Who's Seen Some Stuff");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        // GAP: Nimble ("can't be blocked by creatures with power 3 or greater")
        //      is not a member of the supported KeywordAbility surface and is a
        //      static block-restriction with no expressible primitive here.
        // GAP: "Kithkin you control have thoughtweft" is a static continuous
        //      keyword-sharing ability — no trigger, no cost, and no Effect /
        //      KeywordAbility variant models the thoughtweft keyword pooling.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
