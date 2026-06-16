//! Gliding Licid — `{2}{U}` 2/2 Licid.
//! "{U}, {T}: This creature loses this ability and becomes an Aura
//! enchantment with enchant creature. Attach it to target creature. You
//! may pay {U} to end this effect." plus the granted static "Enchanted
//! creature has flying."
//!
//! Both halves are the Licid attach/animate mechanic — turning a creature
//! into an Aura with a delayed-revert payment and a host-buff static — and
//! neither is expressible with the demonstrated Effect/ActivationCost
//! surface. The bones (mana cost, color, type, P/T) are faithful; the
//! Licid ability is GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gliding Licid");
    let licid = reg.interner_mut().intern("Licid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(licid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: Licid mechanic — "{U}, {T}: loses this ability and becomes an Aura
    // with enchant creature, attach to target creature; you may pay {U} to end".
    // No Effect models a creature animating into an Aura with a self-removing
    // ability + delayed-revert payment, and the granted "enchanted creature has
    // flying" static rides on that animation. Bones only.
    reg.register(CardDefinition::new(name, chars))
}
