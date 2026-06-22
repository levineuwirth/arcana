//! Quandrix, the Proof — `{4}{G}{U}` 6/6 Legendary Creature — Elder Dragon.
//! Flying, trample.
//! Cascade.
//! Instant and sorcery spells you cast from your hand have cascade.
//!
//! GAP: Cascade is not a usable KeywordAbility variant and its "When you cast
//! this spell" cast-trigger has no battlefield TriggerCondition — recorded but
//! unwired. (Effect::Cascade exists but only as a resolution body, with no
//! cast-trigger hook to fire it.)
//! GAP (static): "Instant and sorcery spells you cast from your hand have
//! cascade" is a pure continuous ability granting an ability to a class of
//! spells — not a triggered/activated ability and not expressible here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quandrix, the Proof");
    let elder = reg.interner_mut().intern("Elder");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
