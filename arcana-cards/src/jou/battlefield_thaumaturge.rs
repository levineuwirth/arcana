//! Battlefield Thaumaturge — `{1}{U}` 2/1 Human Wizard.
//! Heroic — "Each instant and sorcery spell you cast costs {1} less
//! to cast for each creature it targets." and "Whenever you cast a
//! spell that targets this creature, this creature gains hexproof
//! until end of turn."
//!
//! Both printed abilities are unexpressible with the demonstrated API:
//! the cost-reduction is a continuous static (no cost-modification
//! Effect), and the Heroic trigger ("cast a spell targeting this
//! creature") has no matching TriggerCondition variant. Heroic is also
//! not in the supported KeywordAbility surface. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Battlefield Thaumaturge");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Heroic keyword not in supported KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static cost reduction ("costs {1} less for each creature it
    // targets") — no cost-modification Effect.
    // GAP: Heroic trigger ("whenever you cast a spell that targets this
    // creature") — no matching TriggerCondition.
    reg.register(CardDefinition::new(name, chars))
}
