//! Ebondeath, Dracolich — `{2}{B}{B}` 5/2 Legendary Zombie Dragon with Flash and Flying.
//!
//! Oracle:
//! * Flash
//! * Flying
//! * Ebondeath enters tapped.
//! * You may cast this card from your graveyard if a creature not named
//!   Ebondeath, Dracolich died this turn.
//!
//! Only the keyword line is expressible in this card class. The two remaining
//! lines are statics/permissions with no triggered- or activated-ability form
//! in the demonstrated catalog:
//! * "Ebondeath enters tapped" is an enters-the-battlefield replacement with
//!   no Effect/ability primitive here — GAP.
//! * "You may cast this card from your graveyard if …" is a static casting
//!   permission (not a `When`/`Whenever`/`At` trigger and not a `[cost]:`
//!   activated ability) — GAP.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ebondeath, Dracolich");
    let zombie = reg.interner_mut().intern("Zombie");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
