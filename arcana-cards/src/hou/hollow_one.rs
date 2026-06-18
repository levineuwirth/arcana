//! Hollow One — `{5}` 4/4 Artifact Creature — Golem with Cycling {2}.
//! "This spell costs {2} less to cast for each card you've cycled or discarded
//! this turn" is a cost-reduction static (GAP'd).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hollow One");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };

    // GAP: "costs {2} less to cast for each card you've cycled or discarded
    // this turn" — a self-cost-reduction static with no expressible primitive.
    reg.register(CardDefinition::new(name, chars))
}
