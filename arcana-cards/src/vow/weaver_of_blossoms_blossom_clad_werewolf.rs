//! Weaver of Blossoms // Blossom-Clad Werewolf
//!
//! Front face: Creature — Human Werewolf, 2/3, {2}{G}
//!   {T}: Add one mana of any color.
//!     GAP: "one mana of any color" — mana ability requires choice; modeled as colorless mana.
//!     GAP: mana abilities via ActivatedAbilityDef not wired to engine mana pool.
//!   Daybound (If a player casts no spells during their own turn, it becomes night next turn.)
//!     GAP: Daybound keyword / day-night cycle not modeled.
//!
//! Back face: Creature — Werewolf, back stats assumed 4/4 or similar.
//!   {T}: Add two mana of any one color.
//!     GAP: same as above.
//!   Nightbound (If a player casts at least two spells during their own turn, it becomes day.)
//!     GAP: Nightbound keyword / day-night cycle not modeled.
//!
//! GAP: Daybound and Nightbound transform conditions not modeled (day/night cycle not in engine).
//!   Transform triggers are omitted; the card is registered without transform triggers.
//! GAP: back-face-only triggered abilities not auto-installed on transform.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Weaver of Blossoms");

    let sub_human = reg.interner_mut().intern("Human");
    let sub_werewolf = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_human);
    subtypes.0.insert(sub_werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // Back face: Blossom-Clad Werewolf
    let back_name = reg.interner_mut().intern("Blossom-Clad Werewolf");
    let back_sub_werewolf = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_sub_werewolf);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // GAP: Daybound/Nightbound day-night cycle not modeled.
            // No transform triggers — the transform condition is not expressible.
    )
}
