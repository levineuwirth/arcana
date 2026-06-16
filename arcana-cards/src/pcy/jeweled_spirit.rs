//! Jeweled Spirit — `{3}{W}{W}` 3/3 Spirit with Flying.
//! "Sacrifice two lands: This creature gains protection from artifacts
//! or from the color of your choice until end of turn."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jeweled Spirit");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP (activated ability): "Sacrifice two lands: gains protection from
    // artifacts or from the color of your choice until end of turn." — there is
    // no grant-protection effect in the available catalog (Protection isn't in
    // the usable KeywordAbility surface and no GrantKeyword arm covers it).
    reg.register(CardDefinition::new(name, chars))
}
