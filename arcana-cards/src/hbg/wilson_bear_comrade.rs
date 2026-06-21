//! Wilson, Bear Comrade — `{1}{G}` 2/2 Legendary Bear Warrior with Reach,
//! Trample, and Ward {2}.
//! Specialize {4}.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wilson, Bear Comrade");
    let bear = reg.interner_mut().intern("Bear");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);
    subtypes.0.insert(warrior);

    // GAP: Specialize {4} — the specialize activation (pay cost + discard a
    // card, transform into a colored back face) has no ActivatedAbility/Effect
    // expression here; only the SelfSpecializes trigger and Effect::Specialize
    // exist, neither of which models the {4}-cost activation/face swap.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Reach,
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
