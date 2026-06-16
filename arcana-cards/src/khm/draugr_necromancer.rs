//! Draugr Necromancer — `{3}{B}` 4/4 Snow Zombie Cleric.
//! "If a nontoken creature an opponent controls would die, exile that
//! card with an ice counter on it instead."
//! "You may cast spells from among cards in exile your opponents own
//! with ice counters on them, and you may spend mana from snow sources
//! as though it were mana of any color to cast those spells."
//!
//! Both abilities are pure STATIC continuous/replacement effects with
//! no trigger word and no activation cost, so neither maps to a
//! TriggeredAbilityDef or ActivatedAbilityDef. Both are GAP'd; only the
//! bones (incl. the Snow supertype) are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Draugr Necromancer");
    let zombie = reg.interner_mut().intern("Zombie");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static replacement "if a nontoken creature an opponent
    // controls would die, exile it with an ice counter instead" and the
    // static cast-from-exile permission are continuous statics with no
    // trigger/activation shape. Both omitted.
    reg.register(CardDefinition::new(name, chars))
}
