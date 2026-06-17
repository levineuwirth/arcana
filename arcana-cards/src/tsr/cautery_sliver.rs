//! Cautery Sliver — `{R}{W}` 2/2 Sliver.
//! "All Slivers have '{1}, Sacrifice this permanent: This permanent deals 1
//!  damage to any target.'"
//! "All Slivers have '{1}, Sacrifice this permanent: Prevent the next 1 damage
//!  that would be dealt to target player, planeswalker, or Sliver creature this
//!  turn.'"
//! Both lines are ability-GRANTING statics (they bestow an activated ability on
//! every Sliver, not on this card alone). There is no expressible primitive to
//! grant a board-wide activated ability as a static, so both are GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cautery Sliver");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "All Slivers have '{1}, Sacrifice: deal 1 to any target'" — board-wide
    //      activated-ability grant (static), not expressible.
    // GAP: "All Slivers have '{1}, Sacrifice: prevent next 1 damage ...'" — same.
    reg.register(CardDefinition::new(name, chars))
}
