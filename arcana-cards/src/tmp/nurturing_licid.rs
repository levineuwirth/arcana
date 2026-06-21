//! Nurturing Licid — `{1}{G}` 1/1 Licid.
//!
//! GAP: "{G}, {T}: This creature loses this ability and becomes an Aura
//! enchantment with enchant creature. Attach it to target creature. You
//! may pay {G} to end this effect." — the Licid creature->Aura transform
//! with an ability-loss + ongoing-payment rider is not expressible with
//! the demonstrated Effect / ActivationCost surface.
//!
//! GAP: "{G}: Regenerate enchanted creature." — references the
//! enchanted-creature target produced by the (unexpressible) Licid Aura
//! state above; there is no way to resolve "enchanted creature" without
//! that machinery, so the whole activated ability is GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nurturing Licid");
    let licid = reg.interner_mut().intern("Licid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(licid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
