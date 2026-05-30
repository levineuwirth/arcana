//! Disciple of Freyalise // Garden of Freyalise
//!
//! Front face: {3}{G}{G}{G} Creature — Elf Druid 3/3
//! "When this creature enters, you may sacrifice another creature. If you do, you gain X life
//! and draw X cards, where X is that creature's power."
//!
//! Back face: Land
//! "As this land enters, you may pay 3 life. If you don't, it enters tapped."
//! {T}: Add {G}.
//!
//! GAP: Front-face ETB "you may sacrifice a creature; if you do, gain X life and draw X cards
//!      where X is that creature's power" — sacrifice-choice cost (OptionalPaymentKind has no
//!      Sacrifice variant); the X-scaling off the sacrificed creature's power is also not
//!      expressible without knowing which creature was sacrificed. Whole ETB effect GAP'd.
//! GAP: Back-face land "enters tapped unless you pay 3 life" is an enters-tapped replacement
//!      effect conditioned on an OptionalPayment; replacement effects not yet modeled.
//! GAP: Back-face {T}: Add {G} — land mana ability on MDFC back face (ActivatedAbilityDef with
//!      face_gate: Some(1)) not modeled here since the land tap ability needs to be an
//!      ActivatedAbilityDef but the back face has no printed mana cost to register as a spell.
//!      The land mana ability is deferred (MDFC land backs are recognized by the engine as lands
//!      for play purposes, but bespoke mana ability author is needed).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Disciple of Freyalise");
    let elf_sub = reg.interner_mut().intern("Elf");
    let druid_sub = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf_sub);
    subtypes.0.insert(druid_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Back face: Garden of Freyalise — Land
    let back_name = reg.interner_mut().intern("Garden of Freyalise");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::LAND.into(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: ETB "may sacrifice another creature; if you do, gain X life and draw X cards
            //      where X is that creature's power" — sacrifice-choice OptionalPaymentKind not
            //      expressible; whole ETB effect omitted.
            .with_mdfc_back(back)
    )
}
