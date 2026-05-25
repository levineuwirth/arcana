//! Arrogant Bloodlord — `{1}{B}{B}` 4/4 black Vampire Knight creature.
//! "Whenever this creature blocks or becomes blocked by a creature with power 1 or less,
//! destroy this creature at end of combat."
//! GAP: no trigger condition for "blocks or becomes blocked by creature with power 1 or less"
//! specifically. Using SelfBecomesBlocked as closest match. The delayed end-of-combat
//! destroy is approximated with DelayedAction (NextEndStep is closest available).

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arrogant Bloodlord");
    let vampire = reg.interner_mut().intern("Vampire");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger condition should fire only when blocker/blocked creature has
                // power 1 or less; SelfBecomesBlocked is the closest available.
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: blocked_schedule_destroy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn blocked_schedule_destroy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should be "at end of combat" but NextEndStep is the closest available when.
    vec![Effect::DelayedAction {
        source: trig.source,
        controller: trig.controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::Sacrifice,
    }]
}
