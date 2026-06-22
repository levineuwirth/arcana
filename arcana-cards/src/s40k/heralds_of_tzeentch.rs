//! Heralds of Tzeentch — `{4}{U}` 3/3 blue Demon with Flying.
//!
//! Oracle text:
//! * Flying.
//! * Cascade.
//!
//! Implemented: the Flying keyword.
//!
//! GAP: Cascade is not in the engine's `KeywordAbility` surface, and its
//! "when you cast this spell, …" cast trigger has no representation in
//! the public `TriggerCondition` set (only `SpellCast { caster }`, which
//! fires on every spell its controller casts), so Cascade is omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heralds of Tzeentch");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
