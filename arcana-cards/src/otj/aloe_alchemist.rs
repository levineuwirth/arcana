//! Aloe Alchemist — `{1}{G}` 3/2 Creature — Plant Warlock.
//! Trample. "When this card becomes plotted, target creature gets +3/+2 and
//! gains trample until end of turn." Plot {1}{G}.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetFilter, TargetCount, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aloe Alchemist");
    let plant = reg.interner_mut().intern("Plant");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(warlock);

    // GAP: keyword "Plot" — not in the modeled KeywordAbility surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — "when this card becomes plotted" has no dedicated
            // TriggerCondition; approximated as SelfEntersBattlefield so the
            // pump payload still fires on a comparable event.
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: pump_target,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn pump_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 3,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Trample],
    }]
}
