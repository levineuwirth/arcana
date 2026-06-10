//! Tolarian Entrancer — `{1}{U}` 1/1 blue Human Wizard.
//! "Whenever this creature becomes blocked by a creature, gain control of that creature
//! at end of combat."
//! "That creature" is recovered via `script::blockers_of` (every blocker —
//! the oracle trigger fires once per blocking creature).
//! GAP: "at end of combat" — ChangeControl is immediate and permanent; no
//! end-of-combat-delayed control-change variant exists. Using ChangeControl
//! at trigger resolution as best effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
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
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "at end of combat" — using ChangeControl (immediate, permanent)
    // as best effort; no end-of-combat-delayed control-change variant.
    script::blockers_of(state, trig.source)
        .into_iter()
        .map(|blocker| Effect::ChangeControl {
            target: blocker,
            new_controller: trig.controller,
        })
        .collect()
}
