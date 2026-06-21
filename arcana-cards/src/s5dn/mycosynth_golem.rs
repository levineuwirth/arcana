//! Mycosynth Golem — `{11}` 4/5 Artifact Creature — Golem.
//!
//! Oracle:
//! * Affinity for artifacts. (This spell costs {1} less to cast for each
//!   artifact you control.)
//! * Artifact creature spells you cast have affinity for artifacts.
//!
//! Both lines are static cost-reduction abilities. `Affinity` is not in the
//! usable `KeywordAbility` surface, and there is no Effect / ActivatedAbility
//! form for a cost-reduction static, so both are GAP'd. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mycosynth Golem");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{11}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: keyword "Affinity for artifacts" — not in the implemented
        // KeywordAbility surface and no cost-reduction static form.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "Artifact creature spells you cast have affinity for
    // artifacts" (cost-reduction granting static; not expressible).
    reg.register(CardDefinition::new(name, chars))
}
