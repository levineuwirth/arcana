//! Grenzo, Havoc Raiser — `{R}{R}` 2/2 Legendary Goblin Rogue.
//!
//! "Whenever a creature you control deals combat damage to a player,
//! choose one —
//!  • Goad target creature that player controls.
//!  • Exile the top card of that player's library. Until end of turn,
//!    you may cast that card and you may spend mana as though it were
//!    mana of any color to cast that spell."
//!
//! Decomposition: one triggered ability (a creature you control deals
//! combat damage to a player). The "choose one" modal payload is NOT
//! expressible on a triggered ability (modal dispatch exists only for
//! spell abilities), so we implement the first mode (Goad a creature the
//! damaged player controls) and GAP the modal selection + the second
//! mode (impulse-exile from that player's library with any-color cast
//! permission).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grenzo, Havoc Raiser");
    let goblin = reg.interner_mut().intern("Goblin");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: modal "choose one" is not expressible on a triggered
            // ability (only spell abilities dispatch modes). Mode 1
            // (Goad) is implemented below; mode 2 (exile top card of the
            // damaged player's library + any-color cast permission) is
            // GAP'd — the trigger always Goads instead of offering the
            // choice. The target filter is also a fidelity widening: we
            // can't constrain "a creature THAT PLAYER controls" because
            // the damaged player isn't known until resolution, so it
            // targets any creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: goad_target_creature,
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

fn goad_target_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Goad {
        target: *id,
        goader: trig.controller,
        duration: Duration::EndOfTurn,
    }]
}
