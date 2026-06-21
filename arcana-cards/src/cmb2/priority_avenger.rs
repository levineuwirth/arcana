//! Priority Avenger — `{3}{W}` 3/4 Creature — Bird Wizard with Flying.
//!
//! Oracle:
//! * Flying — base keyword.
//! * Players can't cast instant spells unless a spell or ability is on the
//!   stack. — a STATIC casting-restriction (a rule-altering static); not a
//!   triggered/activated ability and not expressible here. GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Priority Avenger");
    let bird = reg.interner_mut().intern("Bird");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static — "Players can't cast instant spells unless a spell or ability
    // is on the stack" (rule-altering casting restriction).
    reg.register(CardDefinition::new(name, chars))
}
