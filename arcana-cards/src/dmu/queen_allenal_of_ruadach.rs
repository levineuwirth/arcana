//! Queen Allenal of Ruadach — `{G}{W}{W}` */* Legendary Elf Noble.
//! Queen Allenal of Ruadach's power and toughness are each equal to the
//! number of creatures you control.
//! If one or more creature tokens would be created under your control,
//! those tokens plus a 1/1 white Soldier creature token are created
//! instead.
//!
//! Both abilities are GAP'd:
//!  * The CDA "power and toughness equal to the number of creatures you
//!    control" has no dynamic-P/T surface for creatures-with-abilities;
//!    bones recorded as */* via PtValue::Star.
//!  * The token-creation replacement ("those tokens plus a Soldier are
//!    created instead") has no available replacement primitive.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Queen Allenal of Ruadach");
    let elf = reg.interner_mut().intern("Elf");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(noble);

    // GAP: CDA P/T = number of creatures you control (recorded as */*).
    // GAP: creature-token-creation replacement (extra 1/1 Soldier).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
