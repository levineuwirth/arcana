//! Drowner of Truth // Drowned Jungle — `{5}{G/U}{G/U}` (colorless via Devoid) Creature — Eldrazi.
//! 7/6. Devoid (this card has no color).
//! Front: When you cast this spell, if {C} was spent to cast it, create two 0/1 colorless
//!   Eldrazi Spawn creature tokens with "Sacrifice this token: Add {C}."
//! Back: Land — enters tapped; {T}: Add {G} or {U}.
//!
//! # GAPs
//! - "If {C} was spent to cast it": condition cannot be checked at effect resolution;
//!   the cast trigger is emitted but without the conditional (fires unconditionally if present).
//!   Since we cannot check the mana-spent condition, the token creation is GAP'd.
//! - Eldrazi Spawn "Sacrifice this token: Add {C}": activated ability on token not authorable
//!   via TokenDefinition (only TriggeredAbilityDef supported).
//! - Back face land "enters tapped" and "{T}: Add {G} or {U}": land activated abilities on
//!   MDFC back are not modeled; back face has spell_ability: None.
//! - Colors: Devoid means ColorSet::colorless() even though cost contains {G/U} symbols.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drowner of Truth");
    let back_name = reg.interner_mut().intern("Drowned Jungle");
    let eldrazi_sub = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi_sub);

    // Devoid: the card has no color despite hybrid mana cost
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G/U}{G/U}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // Back face: Land (enters tapped, {T}: Add {G} or {U})
    // GAP: land ETB-tapped and mana abilities not modeled via mdfc_back
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::colorless(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };

    // GAP: "When you cast this spell, if {C} was spent to cast it, create two 0/1 colorless
    // Eldrazi Spawn tokens with Sacrifice: Add {C}" — mana-spent conditional not checkable;
    // Eldrazi Spawn sacrifice ability not authorable on TokenDefinition.

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(CardFace {
                name: back_name,
                characteristics: back_chars,
                spell_ability: None,
            }),
    )
}
