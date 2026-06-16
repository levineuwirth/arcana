//! Sidar Kondo of Jamuraa — `{2}{G}{W}` 2/5 Legendary Creature — Human
//! Knight (green/white). Keywords: Flanking.
//!
//! Oracle text:
//! * Flanking — base keyword, listed in `keywords`. (Whenever a creature
//!   without flanking blocks this creature, the blocking creature gets
//!   -1/-1 until end of turn.)
//! * "Creatures your opponents control without flying or reach can't
//!   block creatures with power 2 or less." — a PURE static continuous
//!   ability (no trigger word, no cost). Not expressible as a
//!   triggered/activated ability here, so it is GAP'd in `register`.
//! * Partner — a commander-format static keyword, not expressible by the
//!   engine; dropped (it is not a documented `KeywordAbility` variant)
//!   and GAP'd in `register`.
//!
//! Only the Flanking keyword is emitted; the two statics are GAP comments.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sidar Kondo of Jamuraa");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flanking],
        ..Default::default()
    };

    // GAP: static "Creatures your opponents control without flying or reach can't block creatures with power 2 or less." — no static-continuous mechanism
    // GAP: "Partner" — commander-format keyword, not expressible
    reg.register(CardDefinition::new(name, chars))
}
