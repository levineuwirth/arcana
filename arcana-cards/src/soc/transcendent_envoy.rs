//! Transcendent Envoy — `{1}{W}` 1/2 Enchantment Creature — Griffin with
//! Flying.
//! "Aura spells you cast cost {1} less to cast" is a static cost-reduction
//! ability with no engine primitive — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Transcendent Envoy");
    let griffin = reg.interner_mut().intern("Griffin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(griffin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Aura spells you cast cost {1} less to cast" — static cost-reduction,
    // no engine primitive.

    reg.register(CardDefinition::new(name, chars))
}
