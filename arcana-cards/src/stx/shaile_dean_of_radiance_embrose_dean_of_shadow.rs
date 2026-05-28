//! Shaile, Dean of Radiance // Embrose, Dean of Shadow
//!
//! Front: Legendary Creature — Bird Cleric {1}{W} 1/1 (white)
//!   Flying, vigilance
//!   {T}: Put a +1/+1 counter on each creature that entered the battlefield under your control this turn.
//! Back: Legendary Creature — Human Warlock
//!   {T}: Put a +1/+1 counter on another target creature, then Embrose deals 2 damage to that creature.
//!   Whenever a creature you control with a +1/+1 counter on it dies, draw a card.
//! GAP: MDFC back face not modeled (mechanic deferred)
//! GAP: "{T}: Put counter on each creature that entered this turn" activated ability not modeled
//! GAP: Back face activated ability not modeled
//! GAP: "Creature with +1/+1 counter dies" trigger not modeled

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shaile, Dean of Radiance");
    let back_name = reg.interner_mut().intern("Embrose, Dean of Shadow");
    let bird = reg.interner_mut().intern("Bird");
    let cleric = reg.interner_mut().intern("Cleric");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(bird);
    subtypes.insert(cleric);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut back_subtypes = SubtypeSet::new();
    back_subtypes.insert(human);
    back_subtypes.insert(warlock);

    let back_chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes: back_subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_mdfc_back(CardFace {
            name: back_name,
            characteristics: back_chars,
            spell_ability: None,
        }),
    )
}
