//! Lagonna-Band Storyteller — `{3}{W}` 3/4 white Centaur Advisor. "When this creature
//! enters, you may put target enchantment card from your graveyard on top of your
//! library. If you do, you gain life equal to its mana value."
//! GAP: "gain life equal to target's mana value" dynamic amount not in engine;
//! emitting PutOnTopOfLibrary + gain 0 (GAP for mana value).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement, TargetChoice};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lagonna-Band Storyteller");
    let centaur = reg.interner_mut().intern("Centaur");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_enchantment_to_library,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_enchantment_to_library(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "gain life equal to its mana value" dynamic amount not in engine
    vec![Effect::PutOnTopOfLibrary { target: *id }]
}
