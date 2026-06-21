//! Kitsune Ace — `{1}{W}` 2/2 Fox Pilot.
//! Whenever a Vehicle you control attacks, choose one —
//!   • That Vehicle gains first strike until end of turn.
//!   • Untap this creature.
//!
//! No keyword line. Triggered abilities have no modal "choose one"
//! machinery in the demonstrated API (modal is a spell-ability feature),
//! so only the first mode is implemented — the attacking Vehicle gains
//! first strike until end of turn. The "Untap this creature" mode and the
//! player's choice between them are GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kitsune Ace");
    let fox = reg.interner_mut().intern("Fox");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    subtypes.0.insert(pilot);

    let vehicle_filter =
        script::subtype_filter(reg, "Vehicle").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: vehicle_filter,
            },
            intervening_if: None,
            // GAP (modal): triggered abilities have no "choose one" machinery.
            // Only mode 1 is implemented; the "Untap this creature" mode and
            // the player's choice between modes are omitted.
            effect: vehicle_gains_first_strike,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn vehicle_gains_first_strike(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.attacking_creature() else { return Vec::new(); };
    vec![Effect::GrantKeyword {
        target: id,
        keyword: KeywordAbility::FirstStrike,
        duration: Duration::EndOfTurn,
    }]
}
