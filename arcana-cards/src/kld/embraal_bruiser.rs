//! Embraal Bruiser — `{1}{B}` 3/1 Human Warrior.
//!
//! "This creature enters tapped. This creature has menace as long as
//! you control an artifact."
//!
//! Both lines are statics with no demonstrated primitive: "enters
//! tapped" is an enters-the-battlefield replacement and the conditional
//! menace is a continuous self-grant gated on board state. Neither maps
//! to a TriggeredAbilityDef/ActivatedAbilityDef, so both are GAP'd and
//! only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Embraal Bruiser");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "enters tapped" (ETB replacement) — no demonstrated primitive.
    // GAP: "has menace as long as you control an artifact" — conditional
    //       continuous static, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
