//! Jukai Naturalist — `{G}{W}` 2/2 Enchantment Creature — Human Monk with
//! Lifelink.
//!
//! Oracle:
//! * Lifelink (keyword).
//! * Enchantment spells you cast cost {1} less to cast. (static cost
//!   reduction — GAP'd below.)
//!
//! Only the Lifelink keyword is expressible. The cost-reduction static has no
//! triggered/activated representation in this shape and no cost-reduction
//! effect primitive, so it is GAP'd. The card is built as an Enchantment
//! Creature.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jukai Naturalist");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);

    // GAP: "Enchantment spells you cast cost {1} less to cast." — a static
    // cost-reduction ability with no triggered/activated representation and no
    // cost-reduction effect primitive in this shape.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
