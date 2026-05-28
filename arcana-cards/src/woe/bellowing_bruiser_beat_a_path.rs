//! Bellowing Bruiser // Beat a Path — `{4}{R}` / `{2}{R}` Adventure
//!
//! Creature: `{4}{R}` Creature — Ogre (4/4)
//!   Haste.
//!
//! Adventure: `{2}{R}` Sorcery — Beat a Path
//!   Up to two target creatures can't block this turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bellowing Bruiser");
    let ogre_sub = reg.interner_mut().intern("Ogre");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Haste],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Beat a Path");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Up to two target creatures can't block this turn.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Creature,
            count: TargetCount::UpTo(2),
            controller: None,
        }],
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    entry.targets.targets.iter()
        .filter_map(|t| {
            if let TargetChoice::Object(id) = t {
                Some(Effect::ForbidAttacking { target: *id, duration: Duration::EndOfTurn })
            } else {
                None
            }
        })
        .collect()
}
