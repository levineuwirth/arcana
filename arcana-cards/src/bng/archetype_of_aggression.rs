//! Archetype of Aggression — `{1}{R}{R}` 3/2 Enchantment Creature — Human Warrior.
//!
//! Creatures you control have trample.
//! Creatures your opponents control lose trample and can't have or gain trample.
//!
//! GAP: both abilities are pure static continuous abilities (no trigger word,
//! no activation cost). This MultiAbilityCreature shape only decomposes
//! triggered ('When/Whenever/At …') and activated ('[cost]: …') abilities;
//! a board-wide keyword-granting/keyword-denying static is not expressible as
//! a TriggeredAbilityDef or ActivatedAbilityDef. The bones (an Enchantment
//! Creature 3/2) are emitted; both statics are gaps.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archetype of Aggression");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
