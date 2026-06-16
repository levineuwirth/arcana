//! Avatar of Might — `{6}{G}{G}` 8/8 green Avatar with Trample.
//! "If an opponent controls at least four more creatures than you, this spell
//! costs {6} less to cast." (GAP — cost-reduction static, no primitive.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Avatar of Might");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "If an opponent controls at least four more creatures than you, this
    // spell costs {6} less to cast." — conditional cost-reduction static; no
    // expressible primitive on this card class.
    reg.register(CardDefinition::new(name, chars))
}
