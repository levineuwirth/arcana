//! Argivian Phalanx — `{5}{W}` 4/4 Human Kor Soldier with Vigilance.
//! Affinity for creatures; Vigilance.
//!
//! Affinity is a cost-reduction keyword not expressible for this card
//! class, so it is recorded as a GAP. Vigilance is faithful.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Argivian Phalanx");
    let human = reg.interner_mut().intern("Human");
    let kor = reg.interner_mut().intern("Kor");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(kor);
    subtypes.0.insert(soldier);

    // GAP: "Affinity for creatures" — cost reduction not expressible here.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
