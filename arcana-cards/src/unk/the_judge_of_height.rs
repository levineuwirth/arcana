//! The Judge of Height — `{3}{R}{W}` 0/6 Legendary Giant Wizard with Trample.
//! "The Judge of Height gets +1/+0 for each creature on the battlefield that
//!  it's taller than. (A creature's height = its toughness plus its mana value.)"
//!
//! GAP: the +1/+0-per-shorter-creature clause is a STATIC continuous
//!      characteristic-defining self-buff keyed on a bespoke "height"
//!      comparison (toughness + mana value). There is no static self-pump or
//!      height-comparison primitive in the usable catalog, so only the Trample
//!      keyword and bones are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Judge of Height");
    let giant = reg.interner_mut().intern("Giant");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
