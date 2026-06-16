//! Fortune Thief — `{4}{R}` 0/1 Human Rogue with Morph {R}{R}.
//! "Damage that would reduce your life total to less than 1 reduces it
//! to 1 instead."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fortune Thief");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Morph keyword not in the usable KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };
    // GAP: static replacement "Damage that would reduce your life total to
    // less than 1 reduces it to 1 instead" — a continuous damage-floor
    // replacement effect, not a triggered/activated ability and not
    // expressible as a card-level static here.
    reg.register(CardDefinition::new(name, chars))
}
