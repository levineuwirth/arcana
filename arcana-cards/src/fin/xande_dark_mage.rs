//! Xande, Dark Mage — `{2}{U}{B}` 3/3 Legendary Human Wizard with Menace.
//! "Xande gets +1/+1 for each noncreature, nonland card in your graveyard."
//! Only the keyword line is expressible here; the dynamic static buff is a GAP.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Xande, Dark Mage");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: "+1/+1 for each noncreature, nonland card in your graveyard" is a
    // dynamic continuous static buff (CDA-like P/T), not a triggered or
    // activated ability — not expressible with the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
