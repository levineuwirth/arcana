//! Tolarian Entrancer — `{1}{U}` 1/1 blue Human Wizard.
//! "Whenever this creature becomes blocked by a creature, gain control of that creature
//! at end of combat."
//! GAP: "at end of combat" — ChangeControl is permanent; no ChangeControlEndOfCombat
//! variant exists. Using ChangeControl as best effort (grants permanent control rather
//! than until-end-of-combat).

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Tolarian Entrancer");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: becomes_blocked_gain_control,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn becomes_blocked_gain_control(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "at end of combat" — using ChangeControl (permanent) as best effort;
    // no ChangeControlEndOfCombat variant. Also GAP: "that creature" (the blocker)
    // would need trig.other_combatant(), but SelfBecomesBlocked pairs with
    // other_combatant for the (first) blocker.
    let Some(id) = trig.other_combatant() else { return Vec::new(); };
    vec![Effect::ChangeControl {
        target: id,
        new_controller: trig.controller,
    }]
}
