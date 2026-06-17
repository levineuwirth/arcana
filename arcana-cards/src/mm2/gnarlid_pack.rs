//! Gnarlid Pack — `{1}{G}` 2/2 green Beast.
//!
//! Multikicker {1}{G}. This creature enters with a +1/+1 counter on it
//! for each time it was kicked.
//!
//! Multikicker is not in the usable keyword surface and there is no
//! engine field for "additional cost paid any number of times" / a
//! kicked-count accessor, so the entire kicker mechanic (and the
//! enters-with-counters-per-kick rider) is a GAP. Emitted as faithful
//! vanilla bones.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gnarlid Pack");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Multikicker {1}{G} and "enters with a +1/+1 counter for each
        // time it was kicked" — no kicker cost field nor a kicked-count
        // accessor in the expressible API.
        keywords: vec![] as Vec<KeywordAbility>,
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
