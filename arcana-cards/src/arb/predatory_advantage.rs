//! Predatory Advantage — `{3}{R}{G}` enchantment.
//! "At the beginning of each opponent's end step, if that player
//! didn't cast a creature spell this turn, create a 2/2 green Lizard
//! creature token."
//!
//! An opponent-end-step trigger with a CR 603.4 intervening-if over
//! `script::spells_cast_this_turn`. GAP: the intervening-if reads the
//! FIRST opponent (the documented 2-player read) — in multiplayer
//! "that player" (whose end step it is) has no accessor.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Predatory Advantage");
    let _lizard = reg.interner_mut().intern("Lizard");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: Some(if_opponent_cast_no_creature),
                effect: make_lizard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…if that player didn't cast a creature spell this turn…"
fn if_opponent_cast_no_creature(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // GAP: "that player" (whose end step it is) has no accessor here;
    // the first opponent is the documented 2-player read.
    let Some(opp) = script::opponents(s, you).first().copied() else {
        return false;
    };
    let creature = ObjectFilter::new().with_types(TypeLine::CREATURE.into());
    script::spells_cast_this_turn(s, &creature, opp) == 0
}

/// "…create a 2/2 green Lizard creature token."
fn make_lizard(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(lizard) = reg.interner().lookup("Lizard") else {
        return Vec::new();
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: lizard,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
