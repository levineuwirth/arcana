//! Dross Golem — `{5}` 3/2 Artifact Creature — Golem.
//!
//! Affinity for Swamps. (GAP — cost-reduction static; "Affinity" is not a
//! usable keyword variant.)
//! Fear.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dross Golem");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    // GAP: "Affinity for Swamps (This spell costs {1} less to cast for each
    // Swamp you control.)" — a cost-reduction static; not a usable keyword and
    // no triggered/activated hook expresses it.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Fear],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
