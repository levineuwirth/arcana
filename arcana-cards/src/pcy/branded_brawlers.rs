//! Branded Brawlers — `{R}` 2/2 red Creature — Human Soldier.
//!
//! This creature can't attack if defending player controls an untapped land.
//! This creature can't block if you control an untapped land.
//!
//! Decomposition: both lines are pure static continuous restrictions (no
//! trigger word, no cost) conditioning attack/block legality on board
//! state — neither is expressible as a triggered/activated ability, and
//! there is no demonstrated attack/block-restriction effect, so both are
//! GAP'd. Only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Branded Brawlers");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP: static "This creature can't attack if defending player controls
    // an untapped land." — a conditional attack-restriction continuous
    // ability; no trigger/cost and no demonstrated restriction effect.
    // GAP: static "This creature can't block if you control an untapped
    // land." — a conditional block-restriction continuous ability;
    // likewise unexpressible.
    reg.register(CardDefinition::new(name, chars))
}
