//! Scaleguard Sentinels — `{G}{G}` 2/3 Human Soldier.
//! "As an additional cost to cast this spell, you may reveal a Dragon card
//! from your hand." (GAP — optional additional reveal cost not expressible.)
//! "This creature enters with a +1/+1 counter on it if you revealed a Dragon
//! card or controlled a Dragon as you cast this spell." (GAP — depends on the
//! unmodeled cast-time reveal/control condition.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "As an additional cost to cast this spell, you may reveal a Dragon
// card from your hand" — no optional additional-cost reveal primitive.
// GAP: "enters with a +1/+1 counter if you revealed a Dragon card or
// controlled a Dragon as you cast this spell" — the cast-time condition the
// counter is gated on is unmodeled, so the conditional enters-with cannot be
// expressed.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scaleguard Sentinels");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
