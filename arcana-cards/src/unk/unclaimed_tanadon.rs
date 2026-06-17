//! Unclaimed Tanadon — `{5}{G}{G}` 8/6 artifact Beast.
//! Both abilities are team-based cost/payment modifications (Mirran cost
//! reduction; Phyrexian-mana colored pips) with no expressible primitive —
//! bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unclaimed Tanadon");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: "If you're on the Mirran team, this card costs {1} less to cast." —
    // no team-state nor cost-reduction primitive.
    // GAP: "If you're on the Phyrexian team, the colored mana symbols ... are
    // Phyrexian mana." — no Phyrexian-mana / team-state primitive.
    reg.register(CardDefinition::new(name, chars))
}
