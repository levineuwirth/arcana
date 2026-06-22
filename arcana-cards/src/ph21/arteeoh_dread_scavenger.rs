//! Arteeoh, Dread Scavenger — `{1}{B}{G}{U}` 3/3 Legendary Artifact Creature —
//! Robot.
//! Flying, deathtouch.
//! Whenever Arteeoh deals combat damage to a player, you may exchange control
//! of two other target artifacts. When you do, create a token that's a copy of
//! target artifact you don't control, except it's a 1/1 green Squirrel creature
//! token in addition to its other colors and types.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arteeoh, Dread Scavenger");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Deathtouch],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: combat_damage_exchange,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn combat_damage_exchange(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect variant EXCHANGES control of two permanents (only
    // one-directional ChangeControl), and the reflexive "When you do, create a
    // token copy of target artifact you don't control, except it's a 1/1 green
    // Squirrel in addition to its other colors/types" is a layered
    // copy-with-overrides + reflexive sub-trigger that is not expressible.
    Vec::new()
}
