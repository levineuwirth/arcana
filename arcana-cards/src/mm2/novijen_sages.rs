//! Novijen Sages — `{4}{U}{U}` 0/0 Human Advisor Mutant with Graft 4.
//!
//! Oracle:
//! * Graft 4 — enters with four +1/+1 counters; whenever another
//!   creature enters, you may move a +1/+1 counter onto it. Modeled via
//!   the `KeywordAbility::Graft(4)` keyword (engine-wired).
//! * {1}, Remove two +1/+1 counters from among creatures you control:
//!   Draw a card. — GAP: the cost removes counters "from among
//!   creatures you control" (any of your creatures, chooser's choice),
//!   which is not an expressible ActivationCost — `remove_self_counter`
//!   only takes counters off this card's own source. Modeling only the
//!   {1} would drop the (mandatory) counter cost and make the draw
//!   nearly free, so the whole ability is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Novijen Sages");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Graft(4)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
