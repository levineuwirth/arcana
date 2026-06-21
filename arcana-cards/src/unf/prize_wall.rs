//! Prize Wall — `{1}{U}` 0/4 Wall with Defender.
//! "Defender"
//! "{U}, {T}: You get {TK}." (acquire a Ticket — Un-set mechanic, GAP)
//! "{4}{U}, {T}: You may put a sticker on a nonland permanent you own. Activate
//!  only as a sorcery." (sticker — Un-set mechanic, GAP)
//!
//! Defender is the keyword line. Both activated abilities reference Un-set
//! mechanics (Tickets and stickers) that the engine does not model, so their
//! effects are not expressible (GAP'd).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prize Wall");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    // GAP: "{U}, {T}: You get {TK}." — acquiring a Ticket ({TK}) is an Un-set
    // mechanic not modeled by the engine.
    // GAP: "{4}{U}, {T}: You may put a sticker on a nonland permanent you own."
    // — name-stickers are an Un-set mechanic not modeled by the engine.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
