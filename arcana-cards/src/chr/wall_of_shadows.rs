//! Wall of Shadows — `{1}{B}{B}` 0/1 Wall with Defender.
//! "Prevent all damage that would be dealt to this creature by creatures
//! it's blocking" and the can't-be-targeted-by-Wall-only static are both
//! continuous statics with no triggered/activated/keyword representation
//! in the demonstrated API.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wall of Shadows");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: static "prevent all damage dealt to this creature by creatures
    // it's blocking" — no continuous self-replacement prevention static
    // expressible from the demonstrated API (PreventDamageFrom is an Effect,
    // not a printed static).
    // GAP: static "can't be the target of Wall-only spells/abilities" — no
    // representation in the demonstrated API.

    reg.register(CardDefinition::new(name, chars))
}
