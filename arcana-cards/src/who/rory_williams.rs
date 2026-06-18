//! Rory Williams — `{W}{U}` 3/3 Legendary Human Soldier.
//! First strike, lifelink. Partner with Amy Pond. "The Last Centurion":
//! a cast-from-anywhere-but-exile trigger that exiles it with three time
//! counters + suspend, then investigates.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rory Williams");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    // GAP: "Partner with Amy Pond" / "Partner" — partner is not an available
    // KeywordAbility variant in this surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Lifelink],
        ..Default::default()
    };

    // GAP: "The Last Centurion — When you cast this spell from anywhere other
    // than exile, exile it with three time counters and suspend, then
    // investigate." No trigger condition expresses "when you cast THIS spell
    // (zone-qualified)", and the exile-with-time-counters/grant-suspend body is
    // not expressible. Omitted entirely rather than partially mis-modeled.
    reg.register(CardDefinition::new(name, chars))
}
