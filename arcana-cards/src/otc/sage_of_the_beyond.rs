//! Sage of the Beyond — `{5}{U}{U}` 5/5 Spirit Giant with Flying.
//! Spells you cast from anywhere other than your hand cost {2} less.
//! Foretell {4}{U}.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sage of the Beyond");
    let spirit = reg.interner_mut().intern("Spirit");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(giant);

    // GAP: "Spells you cast from anywhere other than your hand cost {2} less to
    // cast." is a static cost-reduction continuous ability with no Effect/
    // ActivatedAbility expression.
    // GAP: Foretell {4}{U} — Foretell is not a usable KeywordAbility variant
    // and the foretell-exile/alternative-cost mechanic is not modeled.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
