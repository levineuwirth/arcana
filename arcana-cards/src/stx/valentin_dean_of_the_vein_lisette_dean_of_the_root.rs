//! Valentin, Dean of the Vein // Lisette, Dean of the Root
//!
//! Front: Legendary Creature — Vampire Warlock {B} 1/1 (black)
//!   Menace, lifelink
//!   If a nontoken creature an opponent controls would die, exile it instead. When you do, you may pay {2}. If you do, create a 1/1 black and green Pest token with "When this token dies, you gain 1 life."
//! Back: Legendary Creature — Human Druid
//!   Whenever you gain life, you may pay {1}. If you do, put a +1/+1 counter on each creature you control and those creatures gain trample until end of turn.
//! GAP: MDFC back face not modeled (mechanic deferred)
//! GAP: Replacement effect (exile instead of die) not modeled
//! GAP: "Whenever you gain life" trigger not modeled (no TriggerCondition for life gain)
//! GAP: Pest token death trigger (gain 1 life) not modeled

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Valentin, Dean of the Vein");
    let back_name = reg.interner_mut().intern("Lisette, Dean of the Root");
    let vampire = reg.interner_mut().intern("Vampire");
    let warlock = reg.interner_mut().intern("Warlock");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(vampire);
    subtypes.insert(warlock);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Lifelink],
        ..Default::default()
    };

    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut back_subtypes = SubtypeSet::new();
    back_subtypes.insert(human);
    back_subtypes.insert(druid);

    let back_chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
