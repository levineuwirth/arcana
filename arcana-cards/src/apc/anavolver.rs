//! Anavolver — `{3}{G}` 3/3 Volver with Kicker {1}{U} and/or {B}.
//! "If this creature was kicked with its {1}{U} kicker, it enters with two
//! +1/+1 counters on it and with flying. If kicked with its {B} kicker, it
//! enters with a +1/+1 counter and with 'Pay 3 life: Regenerate this creature.'"

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anavolver");
    let volver = reg.interner_mut().intern("Volver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(volver);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Kicker (Scryfall) is not a usable KeywordAbility variant.
        ..Default::default()
    };

    // GAP: kicker-conditional ETB riders ("if kicked with its {1}{U} kicker,
    // enters with two +1/+1 counters and flying"; "if kicked with its {B}
    // kicker, enters with a +1/+1 counter and 'Pay 3 life: Regenerate'") —
    // kicker payment state is not tracked / queryable with the available
    // surface, so the conditional ETB modifications cannot be expressed.
    reg.register(CardDefinition::new(name, chars))
}
