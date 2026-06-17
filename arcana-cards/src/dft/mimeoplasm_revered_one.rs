//! Mimeoplasm, Revered One — `{X}{B}{G}{U}` 0/0 Legendary Ooze.
//! As Mimeoplasm enters, exile up to X creature cards from your
//! graveyard. It enters with three +1/+1 counters for each creature
//! card exiled this way.
//! {2}: Mimeoplasm becomes a copy of target creature card exiled with
//! it, except it's 0/0 and has this ability.
//!
//! The ETB "exile up to X then enter with 3 counters per exiled card"
//! couples X-driven graveyard exile to a tracked count and an
//! enters-with-counters replacement — no expressible primitive. The
//! {2} ability copies a card that was specifically exiled WITH this
//! permanent (a tracked set), also inexpressible. Both are GAP'd; only
//! the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mimeoplasm, Revered One");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);

    // GAP: ETB "exile up to X creature cards from your graveyard; enters
    //      with three +1/+1 counters per card exiled" — X-driven exile
    //      coupled to a per-card enters-with-counters replacement.
    // GAP: "{2}: becomes a copy of target creature card exiled with it,
    //      except 0/0 with this ability" — copy of a tracked
    //      exiled-with-it card.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
