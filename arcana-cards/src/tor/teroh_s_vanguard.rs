//! Teroh's Vanguard — `{3}{W}` 2/3 Human Nomad with Flash.
//! "Threshold — As long as there are seven or more cards in your
//! graveyard, this creature has 'When this creature enters, creatures you
//! control gain protection from black until end of turn.'"
//!
//! Flash is wired. The Threshold ability is a static that conditionally
//! GRANTS an ETB-trigger which itself grants protection from black — and
//! protection is not in the usable Effect/KeywordAbility surface — so the
//! whole Threshold clause is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "Threshold — As long as there are seven or more cards in your
// graveyard, this creature has 'When this creature enters, creatures you
// control gain protection from black until end of turn.'" — a static
// graveyard-gated grant of a conditional ETB trigger whose payoff is
// "gain protection from black", which has no expressible Effect; the
// whole clause is omitted.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teroh's Vanguard");
    let human = reg.interner_mut().intern("Human");
    let nomad = reg.interner_mut().intern("Nomad");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(nomad);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
