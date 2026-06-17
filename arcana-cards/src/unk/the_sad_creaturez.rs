//! The Sad Creaturez — `{R}{G}{W}` 2/4 Legendary Elemental.
//! "Creatures that haven't shared their feelings can't attack or block." (static — GAP)
//! "All creatures have '{2}: …'" (ability-granting static — GAP)
//! "{T}: Add two mana of any one color." (any-one-color choice — GAP)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Sad Creaturez");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Creatures that haven't shared their feelings can't attack or block."
    //   — a board-wide conditional restriction static, not a triggered/activated
    //   ability; relies on a custom "shared feelings" marker not in the engine.
    // GAP: "All creatures have '{2}: …'" — granting an activated ability to all
    //   creatures is an ability-granting static, not expressible here.
    // GAP: "{T}: Add two mana of any one color." — an any-one-color choice mana
    //   ability is not expressible (Effect::AddMana needs a fixed ManaColor).
    reg.register(CardDefinition::new(name, chars))
}
