//! Melira, Sylvok Outcast — `{1}{G}` 2/2 Legendary Human Scout.
//!
//! Oracle:
//! * "You can't get poison counters." — a static replacement on the
//!   controller; no triggered/activated form. GAP.
//! * "Creatures you control can't have -1/-1 counters put on them." —
//!   a static counter-prohibition replacement. GAP.
//! * "Creatures your opponents control lose infect." — a static
//!   ability-removal continuous effect. GAP.
//!
//! All three lines are pure static continuous/replacement effects with
//! no trigger word and no activation cost, none expressible with the
//! triggered/activated surface available to this card class. The card
//! is emitted with faithful bones and no abilities.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Melira, Sylvok Outcast");
    let human = reg.interner_mut().intern("Human");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: "You can't get poison counters." — static replacement on you.
        // GAP: "Creatures you control can't have -1/-1 counters put on them." — static counter-prohibition replacement.
        // GAP: "Creatures your opponents control lose infect." — static ability-removal continuous effect.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
