//! Howlpack Piper // Wildsong Howler — `{3}{G}` Human Werewolf 2/2.
//! Front face: This spell can't be countered. {1}{G}, {T}: You may put a
//! creature card from your hand onto the battlefield. If it's a Wolf or
//! Werewolf, untap this creature. Activate only as a sorcery.
//! Daybound (GAP: day/night not modeled).
//! Back face (Wildsong Howler): Whenever this creature enters or transforms
//! into Wildsong Howler, look at the top six cards of your library. You may
//! reveal a creature card from among them and put it into your hand. Put the
//! rest on the bottom of your library in a random order.
//! Nightbound (GAP: day/night not modeled).
//!
//! GAP: "This spell can't be countered" is a static property, not expressible.
//! GAP: "{1}{G}, {T}: put a creature card from your hand onto the battlefield"
//! — no Effect::PutFromHandToBattlefield variant; omitted.
//! GAP: "If it's a Wolf or Werewolf, untap this creature" conditional untap; omitted.
//! GAP: Daybound/Nightbound keywords not in engine keyword set.
//! GAP: back-face-only triggered ability not modeled
//! (Whenever this creature enters or transforms into Wildsong Howler, look at
//! the top six cards... — ETB/transform-into trigger on back face only).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Howlpack Piper");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Daybound keyword not in engine keyword set
        // GAP: "This spell can't be countered" static property not expressible
        // GAP: {1}{G}, {T}: put creature from hand to battlefield — no API variant
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Wildsong Howler");
    let werewolf_back_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            // GAP: Nightbound keyword not in engine keyword set
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: back-face-only triggered ability not modeled
    // (Whenever this creature enters or transforms into Wildsong Howler,
    // look at the top six cards of your library — DigTopN creature filter)

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
