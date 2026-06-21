//! Cephalopod Sentry — `{2}{W}{U}` */5 Artifact Creature — Phyrexian Squid.
//!
//! Oracle:
//! * Flying
//! * Cephalopod Sentry's power is equal to the number of artifacts you control.
//!
//! The variable power is a characteristic-defining ability (CR 604.3): a static
//! that sets power, not a triggered/activated ability. It is modeled with
//! `PtValue::Star` (the `*`) and the count-driven CDA is GAP'd, as the
//! demonstrated MultiAbilityCreature surface has no static-CDA hook.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cephalopod Sentry");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let squid = reg.interner_mut().intern("Squid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(squid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        // GAP: power = number of artifacts you control (static CDA); left as `*`.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
