//! Wurmcoil Engine — `{6}` 6/6 Artifact Creature — Phyrexian Wurm with
//! Deathtouch and Lifelink.
//! "When this creature dies, create a 3/3 colorless Phyrexian Wurm artifact
//!  creature token with deathtouch and a 3/3 colorless Phyrexian Wurm
//!  artifact creature token with lifelink."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wurmcoil Engine");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: make_wurm_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_wurm_tokens(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let phyrexian = reg.interner().lookup("Phyrexian").unwrap_or_default();
    let wurm = reg.interner().lookup("Wurm").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(wurm);

    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: wurm,
                colors: ColorSet::colorless(),
                types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(3)),
                toughness: Some(PtValue::Fixed(3)),
                keywords: vec![KeywordAbility::Deathtouch],
                abilities: vec![],
            },
        },
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: wurm,
                colors: ColorSet::colorless(),
                types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                subtypes,
                power: Some(PtValue::Fixed(3)),
                toughness: Some(PtValue::Fixed(3)),
                keywords: vec![KeywordAbility::Lifelink],
                abilities: vec![],
            },
        },
    ]
}
