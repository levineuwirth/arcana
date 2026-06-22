//! Broodstar — `{8}{U}{U}` */* blue Beast with Flying and Affinity for artifacts.
//!
//! "Affinity for artifacts (This spell costs {1} less to cast for each artifact
//!  you control.)
//!  Flying
//!  Broodstar's power and toughness are each equal to the number of artifacts
//!  you control."
//!
//! Flying is a base keyword. Affinity is a cost-reduction keyword with no
//! expressible variant — GAP. The characteristic-defining "power and toughness
//! each equal to the number of artifacts you control" has no registration hook
//! for a CDA P/T static, so P/T is left as `*` (PtValue::Star) and the
//! computing static is GAP'd (cf. Phyrexian Broodstar).

// GAP (keyword): Affinity for artifacts — cost reduction not expressible.
// GAP (CDA): "power and toughness are each equal to the number of artifacts you
// control" — no registration hook for a characteristic-defining P/T static.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Broodstar");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
