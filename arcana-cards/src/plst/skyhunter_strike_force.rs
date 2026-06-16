//! Skyhunter Strike Force — `{2}{W}` 2/2 Cat Knight with Flying.
//! Melee — Whenever this creature attacks, it gets +1/+1 until end of turn for each
//!   opponent you attacked this combat. (GAP — Melee not supported; no "opponents
//!   attacked this combat" accessor)
//! Lieutenant — As long as you control your commander, other creatures you control
//!   have melee. (GAP — static commander-conditional anthem)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyhunter Strike Force");
    let cat = reg.interner_mut().intern("Cat");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // Melee and Lieutenant are not in the supported KeywordAbility set — GAP.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
