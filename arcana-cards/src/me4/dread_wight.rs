//! Dread Wight — `{3}{B}{B}` 3/4 Zombie.
//! At end of combat, put a paralyzation counter on each creature blocking
//! or blocked by this creature and tap those creatures. Each doesn't untap
//! during its controller's untap step while it has the counter. Each gains
//! "{4}: Remove a paralyzation counter."
//! GAP: Custom "paralyzation" counter, conditional untap-step prevention, and
//! granting activated abilities to other creatures not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dread Wight");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    // GAP: full oracle not expressible; emitting stub with no activated ability.
    reg.register(CardDefinition::new(name, chars))
}
