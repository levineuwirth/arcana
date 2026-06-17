//! Walking Skyscraper — `{8}` 8/8 Artifact Creature — Construct with Trample.
//! Cost-reduction static and conditional-hexproof static are GAPs.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Walking Skyscraper");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "This spell costs {1} less to cast for each modified creature you
    // control." — a cost-reduction static, not a triggered/activated ability.
    // GAP: "This creature has hexproof as long as it's untapped." — a
    // conditional continuous static, not a triggered/activated ability.

    reg.register(CardDefinition::new(name, chars))
}
