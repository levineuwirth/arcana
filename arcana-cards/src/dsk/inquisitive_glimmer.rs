//! Inquisitive Glimmer — `{W}{U}` 2/3 Enchantment Creature — Fox Glimmer.
//!
//! Enchantment spells you cast cost {1} less to cast.
//! Unlock costs you pay cost {1} less.
//!
//! Both lines are STATIC cost-reduction abilities (no trigger word, no cost) —
//! neither is expressible as a triggered/activated ability with the
//! demonstrated API, so both are GAP'd and only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inquisitive Glimmer");
    let fox = reg.interner_mut().intern("Fox");
    let glimmer = reg.interner_mut().intern("Glimmer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    subtypes.0.insert(glimmer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: static "Enchantment spells you cast cost {1} less" — cost-reduction
        // continuous static, not expressible as a triggered/activated ability.
        // GAP: static "Unlock costs you pay cost {1} less" — likewise.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
