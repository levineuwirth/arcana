//! Wurmcoil Broodmother — `{9}` 9/9 Artifact Creature — Wurm with
//! Deathtouch and Lifelink.
//!
//! "When Wurmcoil Broodmother dies, create a Wurmcoil Engine token and
//! a Wurmcoil Larva token." Modeled after the classic Wurmcoil Engine
//! death halves: a 3/3 Phyrexian Wurm artifact-creature token with
//! deathtouch and a 3/3 Phyrexian Wurm artifact-creature token with
//! lifelink.

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
    let name = reg.interner_mut().intern("Wurmcoil Broodmother");
    let wurm = reg.interner_mut().intern("Wurm");
    // Pre-intern token subtype names so the resolver can rebuild them.
    let _engine = reg.interner_mut().intern("Wurmcoil Engine");
    let _larva = reg.interner_mut().intern("Wurmcoil Larva");
    let _phyrexian = reg.interner_mut().intern("Phyrexian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{9}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_make_tokens,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_make_tokens(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let wurm = reg.interner().lookup("Wurm").unwrap_or_default();
    let phyrexian = reg.interner().lookup("Phyrexian").unwrap_or_default();
    let engine_name = reg.interner().lookup("Wurmcoil Engine").unwrap_or_default();
    let larva_name = reg.interner().lookup("Wurmcoil Larva").unwrap_or_default();

    let mut engine_subtypes = SubtypeSet::default();
    engine_subtypes.0.insert(phyrexian);
    engine_subtypes.0.insert(wurm);
    let mut larva_subtypes = SubtypeSet::default();
    larva_subtypes.0.insert(phyrexian);
    larva_subtypes.0.insert(wurm);

    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: engine_name,
                colors: ColorSet::colorless(),
                types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                subtypes: engine_subtypes,
                power: Some(PtValue::Fixed(3)),
                toughness: Some(PtValue::Fixed(3)),
                keywords: vec![KeywordAbility::Deathtouch],
                abilities: vec![],
            },
        },
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: larva_name,
                colors: ColorSet::colorless(),
                types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                subtypes: larva_subtypes,
                power: Some(PtValue::Fixed(3)),
                toughness: Some(PtValue::Fixed(3)),
                keywords: vec![KeywordAbility::Lifelink],
                abilities: vec![],
            },
        },
    ]
}
