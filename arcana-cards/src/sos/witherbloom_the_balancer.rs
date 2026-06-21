//! Witherbloom, the Balancer — `{6}{B}{G}` 5/5 Legendary Elder Dragon.
//!
//! Oracle:
//! * Affinity for creatures (cost reduction)
//! * Flying, deathtouch
//! * Instant and sorcery spells you cast have affinity for creatures.
//!
//! GAP: "Affinity" is a cost-reduction keyword Scryfall lists but the
//! demonstrated KeywordAbility surface does NOT include an Affinity
//! variant — it is not in the usable keyword set, so it is omitted.
//! GAP: "Instant and sorcery spells you cast have affinity for creatures"
//! is a static cost-reduction-granting ability with no triggered /
//! activated / Effect form in the demonstrated API.
//! Only the Flying + Deathtouch keywords are expressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Witherbloom, the Balancer");
    let elder = reg.interner_mut().intern("Elder");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
