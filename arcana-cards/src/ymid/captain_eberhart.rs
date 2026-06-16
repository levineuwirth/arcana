//! Captain Eberhart — `{1}{W}` 1/2 Legendary Human Soldier with Double strike.
//! "Spells cast from among cards you drew this turn cost {1} less to cast.
//! Spells cast from among cards your opponents drew this turn cost {1} more to cast."
//!
//! Double strike is expressible. Both cost-modification statics are GAP'd: there is
//! no cost-reduction / cost-increase modifier representation in the demonstrated API.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Captain Eberhart");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    // GAP: "Spells cast from among cards you drew this turn cost {1} less" — no
    //      cost-reduction static modifier.
    // GAP: "Spells cast from among cards your opponents drew this turn cost {1} more"
    //      — no cost-increase static modifier.
    reg.register(CardDefinition::new(name, chars))
}
