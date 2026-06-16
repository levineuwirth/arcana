//! Basalt Golem — `{5}` 2/4 Golem (artifact creature).
//! "This creature can't be blocked by artifact creatures." — a
//! restricted-blocker static with no representable primitive → GAP'd.
//! "Whenever this creature becomes blocked by a creature, that
//! creature's controller sacrifices it at end of combat. If the player
//! does, they create a 0/2 colorless Wall artifact creature token with
//! defender." — the delayed sacrifice of the blocker is scheduled
//! (NextEndStep, a fidelity gap vs. "end of combat"); the conditional
//! "if the player does, create a Wall" cannot be tied to the
//! sacrifice resolving → GAP'd.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
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
    let name = reg.interner_mut().intern("Basalt Golem");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBecomesBlocked,
            intervening_if: None,
            effect: blocker_sacrificed,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn blocker_sacrificed(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If the player does, they create a 0/2 Wall token" — the
    // Wall creation is conditioned on the delayed sacrifice resolving,
    // which cannot be chained, so only the scheduled sacrifice of the
    // blocker is expressed (at the next end step, a fidelity gap vs.
    // "at end of combat").
    let Some(blocker) = trig.other_combatant() else {
        return Vec::new();
    };
    let Some(controller) = state.objects.get(blocker).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::DelayedAction {
        source: blocker,
        controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::Sacrifice,
    }]
}
