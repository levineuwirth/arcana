//! Guardian Archon — `{4}{W}{W}` 5/5 white Archon with Flying.
//! "As this creature enters, secretly choose an opponent." is a hidden
//! as-enters choice with no demonstrated primitive (GAP). The activated
//! "Reveal the player you chose: You and target permanent you control
//! each gain protection from the chosen player until end of turn.
//! Activate only once." grants protection (a quality not in the
//! available keyword surface and with no demonstrated grant Effect) and
//! is GAP'd as well. Only Flying and the bones remain.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Guardian Archon");
    let archon = reg.interner_mut().intern("Archon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(archon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "As this creature enters, secretly choose an opponent." — a
    // hidden as-enters choice with no demonstrated primitive.
    // GAP: "Reveal the player you chose: You and target permanent you
    // control each gain protection from the chosen player until end of
    // turn. Activate only once." — protection grant not expressible.
    reg.register(CardDefinition::new(name, chars))
}
