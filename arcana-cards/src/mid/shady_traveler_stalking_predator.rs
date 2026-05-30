//! Shady Traveler // Stalking Predator — `{2}{B}` Human Werewolf 2/3 (front).
//!
//! Front face: Menace. Daybound (not modeled).
//! Back face: Stalking Predator — Werewolf 2/3. Menace. Nightbound (not modeled).
//!
//! GAP: Daybound/Nightbound keywords are not in the usable keyword surface;
//! omitted from both faces.
//! GAP: Day/night cycle and the precise transform conditions ("no spells cast
//! last turn" / "two spells cast this turn") are not modeled. No transform
//! trigger is wired because neither condition can be expressed with the
//! available TriggerCondition API.
//! GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shady Traveler");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // Back face: Stalking Predator — Werewolf 2/3 with Menace.
    let back_name = reg.interner_mut().intern("Stalking Predator");
    let werewolf_sub2 = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Menace],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: No transform trigger wired — Daybound/Nightbound day/night
    // cycle conditions are not expressible with available TriggerCondition
    // variants.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
