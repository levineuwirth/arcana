//! Lifeline — `{5}` artifact.
//! "Whenever a creature dies, if another creature is on the battlefield,
//! return the first card to the battlefield under its owner's control at
//! the beginning of the next end step."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lifeline");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: Some(if_another_creature),
                effect: return_the_dead,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// Intervening-if: "if another creature is on the battlefield" — at least
/// one creature remains (the dead one has already left).
fn if_another_creature(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::count_matching(s, &ObjectFilter::creature(), you) >= 1
}

/// "…return the first card to the battlefield under its owner's control at
/// the beginning of the next end step."
fn return_the_dead(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(dead) = trig.dying_object() else {
        return Vec::new();
    };
    // GAP: timing fidelity — the return happens immediately on resolution
    // rather than "at the beginning of the next end step" (DelayedAction has
    // no return-from-GRAVEYARD action), and "under its owner's control" is
    // whatever ReturnFromGraveyardToBattlefield does by default.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: dead }]
}
