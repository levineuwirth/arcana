//! Jodah, Archmage Eternal — `{1}{U}{R}{W}` 4/3 Legendary Human Wizard
//! with Flying.
//!
//! * Flying
//! * You may pay {W}{U}{B}{R}{G} rather than pay the mana cost for
//!   spells you cast. (Static alternative-cost replacement — GAP'd:
//!   no Effect/Static surface for casting-cost substitution.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jodah, Archmage Eternal");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "You may pay {W}{U}{B}{R}{G} rather than pay the mana cost
    // for spells you cast" — alternative-cost replacement on casting,
    // not expressible as a triggered/activated ability or Effect.

    reg.register(CardDefinition::new(name, chars))
}
