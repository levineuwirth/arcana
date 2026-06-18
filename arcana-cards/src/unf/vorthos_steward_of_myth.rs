//! Vorthos, Steward of Myth — `{1}{R}` 1/3 Legendary Creature — Human Gamer.
//!
//! "As Vorthos, Steward of Myth enters, choose a named Magic character.
//! Each spell you cast with the chosen character in its name, flavor text,
//! or art costs {W}{U}{B}{R}{G} less to cast. This effect reduces only the
//! amount of colored mana you pay."
//!
//! Both clauses form a single replacement-style cost-reduction static keyed
//! off a player-chosen named character. There is no engine primitive for
//! "choose a named character as this enters" nor for a name/flavor/art-keyed
//! cost reduction, so the whole effect is GAP'd. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vorthos, Steward of Myth");
    let human = reg.interner_mut().intern("Human");
    let gamer = reg.interner_mut().intern("Gamer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(gamer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "As ~ enters, choose a named Magic character" + a cost reduction
    // keyed on that name appearing in a spell's name/flavor text/art is not
    // expressible (no choose-a-name primitive, no name/flavor/art-keyed cost
    // reduction). Bones only.
    reg.register(CardDefinition::new(name, chars))
}
