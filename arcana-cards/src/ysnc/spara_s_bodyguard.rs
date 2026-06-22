//! Spara's Bodyguard — `{G}{W}{U}` 3/3 Rhino Warrior.
//! "When Spara's Bodyguard enters the battlefield, you may choose a
//!  creature card in your hand. If you do, it perpetually gains 'This
//!  creature enters the battlefield with an additional shield counter
//!  on it.' Otherwise, put a shield counter on Spara's Bodyguard."
//! "At the beginning of each combat, Spara's Bodyguard gets +1/+1 until
//!  end of turn for each shield counter among other creatures you
//!  control."
//!
//! Both triggers are wired but their bodies are GAP'd:
//! - The ETB's "perpetually gains" is an Alchemy mechanic with no
//!   engine surface, and its shield-counter fallback is gated on that
//!   unresolvable choice.
//! - The combat pump scales by the SUM of shield counters among other
//!   creatures — there is no script helper that sums counters across a
//!   filtered set (count_matching counts permanents, not counters).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spara's Bodyguard");
    let rhino = reg.interner_mut().intern("Rhino");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_shield_choice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: combat_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_shield_choice(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "perpetually gains ..." is an Alchemy mechanic with no engine
    // surface; the shield-counter fallback is gated on that choice.
    Vec::new()
}

fn combat_pump(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "+1/+1 for each shield counter among other creatures you
    // control" — no helper sums counters across a filtered set.
    Vec::new()
}
