//! Cunning Nightbonder — `{U/B}{U/B}` 2/2 Human Rogue with Flash.
//! "Spells with flash you cast cost {1} less to cast and can't be countered."
//! (static cost-reduction + uncounterable rider — GAP'd)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cunning Nightbonder");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U/B}{U/B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: static "Spells with flash you cast cost {1} less and can't be
    // countered" — a continuous cost-reduction + can't-be-countered static with
    // no trigger or activation; not expressible on this shape.
    reg.register(CardDefinition::new(name, chars))
}
