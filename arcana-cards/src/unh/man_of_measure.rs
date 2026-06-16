//! Man of Measure — `{1}{W}{W}` 2/2 Human Knight.
//!
//! "As long as you're shorter than an opponent, Man of Measure gets +0/+1
//!  and has first strike." (conditional static — not expressible; GAP'd.)
//! "As long as you're taller than an opponent, Man of Measure gets +1/+0."
//!  (conditional static — not expressible; GAP'd.)
//!
//! Both abilities are pure conditional continuous effects with no trigger
//! or cost, so neither maps to a TriggeredAbilityDef / ActivatedAbilityDef.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Man of Measure");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
