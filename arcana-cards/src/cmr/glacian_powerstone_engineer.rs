//! Glacian, Powerstone Engineer — `{5}{U}` 3/6 Legendary Human Artificer.
//! {T}, Tap X untapped artifacts you control: Look at the top X cards of your
//! library. Put one of those cards into your hand and the rest into your
//! graveyard. (GAP — the variable X tap cost is tied to a variable-count dig;
//! tap_other_count is a fixed integer and DigTopN's count cannot be the chosen
//! X, so the whole ability is not expressible.)
//! Partner. (GAP — not in the supported KeywordAbility set.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glacian, Powerstone Engineer");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
