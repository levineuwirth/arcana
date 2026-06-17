//! Doc Aurlock, Grizzled Genius — `{G}{U}` 2/3 Legendary Bear Druid.
//! Spells you cast from your graveyard or from exile cost {2} less to cast.
//! Plotting cards from your hand costs {2} less.
//!
//! Both lines are pure static cost-reduction effects. There is no
//! expressible Effect / triggered / activated primitive for static
//! cost reduction, so both are GAP'd and only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Doc Aurlock, Grizzled Genius");
    let bear = reg.interner_mut().intern("Bear");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Spells you cast from your graveyard or from exile cost {2} less."
    //      Static cost reduction — no expressible primitive.
    // GAP: "Plotting cards from your hand costs {2} less."
    //      Static cost reduction (Plot) — no expressible primitive.
    reg.register(CardDefinition::new(name, chars))
}
