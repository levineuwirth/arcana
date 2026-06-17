//! The Second Doctor — `{2}{W}{U}` 2/4 Legendary Time Lord Doctor.
//! "Players have no maximum hand size." (static — GAP)
//! "How Civil of You — At the beginning of your end step, each player may draw a card.
//!  Each opponent who does can't attack you or permanents you control during their
//!  next turn." (GAP — see below)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Second Doctor");
    let time_lord = reg.interner_mut().intern("Time Lord");
    let doctor = reg.interner_mut().intern("Doctor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(time_lord);
    subtypes.0.insert(doctor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Players have no maximum hand size." — a global rule-altering static.
    // GAP: "How Civil of You — At the beginning of your end step, each player may
    //   draw a card. Each opponent who does can't attack you or permanents you
    //   control during their next turn." — an optional per-player draw whose
    //   conditional rider (a next-turn attack restriction conditioned on whether
    //   that opponent chose to draw) is not expressible with the available effects.
    reg.register(CardDefinition::new(name, chars))
}
