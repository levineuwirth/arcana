//! Misthoof Kirin — `{2}{W}` 2/1 Kirin with Flying and Vigilance.
//! Megamorph {1}{W}. Megamorph is not an expressible KeywordAbility (the
//! face-down cast + flip mechanic is not modeled here), so it is GAP'd; only
//! the Flying/Vigilance keyword line is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Misthoof Kirin");
    let kirin = reg.interner_mut().intern("Kirin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kirin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        // GAP: Megamorph {1}{W} is not an expressible KeywordAbility.
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
