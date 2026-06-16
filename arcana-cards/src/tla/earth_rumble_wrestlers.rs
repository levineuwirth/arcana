//! Earth Rumble Wrestlers — `{3}{R/G}` 3/4 Human Warrior Performer.
//! Reach. "This creature gets +1/+0 and has trample as long as you control
//!  a land creature or a land entered the battlefield under your control
//!  this turn."
//!
//! Reach is a base keyword (and the card is red-green per the {R/G} pip).
//! The conditional self-buff is a static continuous ability with no
//! triggered/activated decomposition and no Effect variant, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "gets +1/+0 and has trample as long as you control a land creature or a
// land entered under your control this turn" — a conditional static buff, not
// expressible as a triggered/activated ability or Effect.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Earth Rumble Wrestlers");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let performer = reg.interner_mut().intern("Performer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    subtypes.0.insert(performer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
