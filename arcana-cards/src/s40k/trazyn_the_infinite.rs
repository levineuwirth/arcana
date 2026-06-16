//! Trazyn the Infinite — `{4}{B}{B}` 4/6 Legendary Artifact Creature —
//! Necron with Deathtouch.
//! "Prismatic Gallery — As long as Trazyn is on the battlefield, it has all
//! activated abilities of all artifact cards in your graveyard."
//!
//! Deathtouch is a keyword. Prismatic Gallery is a static that grafts the
//! activated abilities of every artifact card in your graveyard onto Trazyn
//! — there is no Effect / ability-borrowing surface for it, so it is GAP'd.
//! Bones + Deathtouch faithful.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trazyn the Infinite");
    let necron = reg.interner_mut().intern("Necron");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(necron);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // GAP: Prismatic Gallery — "has all activated abilities of all artifact
    // cards in your graveyard"; no ability-borrowing static surface.
    reg.register(CardDefinition::new(name, chars))
}
