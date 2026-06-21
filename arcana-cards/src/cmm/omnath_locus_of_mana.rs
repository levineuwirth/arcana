//! Omnath, Locus of Mana — `{2}{G}` 1/1 Legendary Elemental.
//! "You don't lose unspent green mana as steps and phases end."
//! "Omnath gets +1/+1 for each unspent green mana you have."
//!
//! Both lines are pure static continuous abilities (no trigger word, no
//! cost): a mana-emptying replacement and a dynamic P/T keyed on the
//! unspent green mana pool. Neither the mana-retention replacement nor a
//! P/T-from-mana-pool reading is expressible with the demonstrated
//! triggered/activated/script API, so both are GAP'd; the bones (and the
//! printed 1/1 base) are kept.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Omnath, Locus of Mana");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "You don't lose unspent green mana as steps and phases end." — a
    // mana-emptying replacement effect, not expressible here.
    // GAP: "Omnath gets +1/+1 for each unspent green mana you have." — a static
    // P/T keyed on the unspent green mana pool, which script:: cannot read.

    reg.register(CardDefinition::new(name, chars))
}
