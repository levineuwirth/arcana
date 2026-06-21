//! Nyssa of Traken — `{3}{U}` 3/4 Legendary Human Scientist.
//!
//! Oracle:
//! * You have no maximum hand size. (Static — not expressible; GAP.)
//! * Sonic Booster — Whenever Nyssa of Traken attacks, sacrifice any number of
//!   artifacts. When you sacrifice one or more artifacts this way, tap up to
//!   that many target creatures and draw that many cards.
//!   GAP: a reflexive trigger whose follow-up count ("that many") is the
//!   player's variable sacrifice count — there is no accessor coupling the
//!   chosen count into the tap/draw payload, so the whole ability is omitted.
//! * Doctor's companion. (Not an expressible keyword — omitted; GAP.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nyssa of Traken");
    let human = reg.interner_mut().intern("Human");
    let scientist = reg.interner_mut().intern("Scientist");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scientist);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "no maximum hand size" static, the Sonic Booster reflexive attack
    //      trigger (variable sacrifice count coupled to tap/draw), and
    //      Doctor's companion are all not expressible.
    reg.register(CardDefinition::new(name, chars))
}
