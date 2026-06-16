//! Ancient Stone Idol — `{10}` 12/12 Artifact Creature — Golem with Flash and Trample.
//! "This spell costs {1} less to cast for each attacking creature." (static — GAP)
//! "When this creature dies, create a 6/12 colorless Construct artifact creature
//! token with trample."

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
    let name = reg.interner_mut().intern("Ancient Stone Idol");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{10}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(12)),
        toughness: Some(PtValue::Fixed(12)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: "This spell costs {1} less to cast for each attacking creature." —
    // dynamic cost reduction static is not in the documented Effect surface.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_make_construct,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_make_construct(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let construct = reg.interner().lookup("Construct").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: construct,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(12)),
            keywords: vec![KeywordAbility::Trample],
            abilities: vec![],
        },
    }]
}
