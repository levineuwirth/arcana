//! Razorkin Needlehead — `{R}{R}` 2/2 Human Assassin.
//!
//! Oracle:
//! * This creature has first strike during your turn. — a conditional
//!   static keyword grant; not expressible as a triggered/activated ability
//!   on this card class, so it is GAP'd.
//! * Whenever an opponent draws a card, this creature deals 1 damage to
//!   them. — `CardDrawn { Opponent }` trigger dealing 1 damage to the
//!   opponent. No typed "drawing player" accessor exists; in a two-player
//!   game the lone opponent is the drawer (multiplayer fidelity gap: hits
//!   the first opponent).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Razorkin Needlehead");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static — "This creature has first strike during your turn."

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CardDrawn {
                player: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: damage_drawing_opponent,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn damage_drawing_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(them) = script::opponents(state, trig.controller).into_iter().next() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(them),
        amount: 1,
    }]
}
