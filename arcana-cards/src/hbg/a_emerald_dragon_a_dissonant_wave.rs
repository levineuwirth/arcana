//! A-Emerald Dragon // A-Dissonant Wave — `{4}{G}{G}` / `{2}{G}` Adventure
//!
//! Creature: `{4}{G}{G}` Creature — Dragon (4/4)
//!   Flying, ward {2}.
//!
//! Adventure: `{2}{G}` Instant — A-Dissonant Wave
//!   Counter target activated or triggered ability from a noncreature source.
//!   (Wired via TargetFilter::AbilityOnStack with a noncreature source_filter;
//!   Effect::Counter handles ability stack entries.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Emerald Dragon");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("A-Dissonant Wave");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Counter target activated or triggered ability from a noncreature source.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::AbilityOnStack {
                activated: true,
                triggered: true,
                source_filter: Some(
                    ObjectFilter::new().without_types(TypeLine::CREATURE.into()),
                ),
            },
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Counter { target: *id }]
}
