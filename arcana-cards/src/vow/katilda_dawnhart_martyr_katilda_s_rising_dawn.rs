//! Katilda, Dawnhart Martyr // Katilda's Rising Dawn
//!
//! Front face: `{1}{W}{W}` Legendary Creature — Spirit Warlock
//! Flying, lifelink, protection from Vampires (GAP: Protection not modeled).
//! Power and toughness each equal to the number of permanents you control
//! that are Spirits and/or enchantments.
//! GAP: this CDA counts a CROSS-AXIS disjunction (Spirit SUBTYPE *or* enchantment
//!   TYPE). A single `ObjectFilter` ANDs its type and subtype predicates, so it
//!   can't express subtype-OR-type; and the registry-free `self_pt_cda`/`custom`
//!   compute paths can't name the interned "Spirit" subtype symbol. Left as a
//!   Fixed(0) placeholder pending a subtype-OR-type count filter (or a
//!   compute-with-interned-symbol hook).
//! Disturb {3}{W}{W} (GAP: Disturb cast-from-graveyard not modeled).
//!
//! Back face (transform): Legendary Enchantment — Aura
//! Enchant creature; enchanted creature has flying, lifelink, protection from
//! Vampires, and gets +X/+X where X = number of Spirits and/or enchantments
//! you control (GAP: all of back-face Aura abilities not modeled via
//! back-face triggered/static system; back-face-only triggered ability not
//! modeled).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Katilda, Dawnhart Martyr");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let warlock_sub = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);
    subtypes.0.insert(warlock_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // GAP: P/T = Spirits and/or enchantments you control — a cross-axis
        // (subtype-OR-type) count not expressible via a single ObjectFilter, and
        // the registry-free CDA compute can't name the "Spirit" subtype symbol.
        // Using Fixed(0) as a placeholder.
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        // GAP: Protection from Vampires not modeled (Protection keyword not in engine surface).
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Katilda's Rising Dawn");
    let aura_sub = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            // GAP: back face is an Aura — enchant creature, flying/lifelink/protection/+X/+X
            // not modeled; back-face-only triggered ability not modeled.
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: Disturb (cast from graveyard transformed) not modeled.
    // GAP: "If Katilda's Rising Dawn would be put into a graveyard from anywhere,
    // exile it instead" — replacement effect not modeled.
    // GAP: back-face-only triggered ability not modeled.

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
