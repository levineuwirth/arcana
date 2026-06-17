//! Forlorn Pseudamma — `{3}{B}` 2/1 Zombie (black) with Intimidate.
//! "Intimidate
//!  Inspired — Whenever this creature becomes untapped, you may pay {2}{B}.
//!  If you do, create a 2/2 black Zombie enchantment creature token."
//!
//! Intimidate is a base keyword. The Inspired ability (a becomes-untapped
//! trigger) has no SelfBecomesUntapped TriggerCondition variant, so it is
//! GAP'd; Inspired itself is not a supported keyword.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forlorn Pseudamma");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Intimidate],
        ..Default::default()
    };

    // GAP: "Inspired — Whenever this creature becomes untapped, you may pay
    // {2}{B} ..." — no SelfBecomesUntapped trigger condition exists.
    reg.register(CardDefinition::new(name, chars))
}
