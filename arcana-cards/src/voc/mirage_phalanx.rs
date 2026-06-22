//! Mirage Phalanx — `{4}{R}{R}` 4/4 Human Soldier.
//! Soulbond. As long as Mirage Phalanx is paired with another creature, each
//! of those creatures has "At the beginning of combat on your turn, create a
//! token that's a copy of this creature, except it has haste and loses
//! soulbond. Exile it at end of combat."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mirage Phalanx");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    // GAP: keyword Soulbond — not in the implemented keyword surface; the
    // pair-on-enters mechanic is unmodeled.
    // GAP: static "as long as paired, each paired creature has '<combat-token
    // copy>'" — a pairing-gated grant of a self-copying triggered ability with
    // an exile-at-end-of-combat rider; not expressible from the demonstrated
    // API (no static ability-grant primitive, no soulbond pairing predicate).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
