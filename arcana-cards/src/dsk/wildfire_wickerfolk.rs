//! Wildfire Wickerfolk — `{R}{G}` 3/2 Artifact Creature — Scarecrow.
//!
//! Haste.
//! Delirium — This creature gets +1/+1 and has trample as long as there are
//! four or more card types among cards in your graveyard.
//!
//! Haste is a base keyword. The Delirium clause is a STATIC conditional
//! continuous ability (no trigger word, no cost) — it is not expressible as a
//! triggered/activated ability with the demonstrated API, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wildfire Wickerfolk");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        // GAP: Delirium static — "+1/+1 and trample as long as four or more
        // card types among cards in your graveyard" is a conditional continuous
        // static ability (no trigger word, no cost); not expressible here.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
