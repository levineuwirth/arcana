//! Sigil of Myrkul — `{2}{B}` enchantment.
//! "At the beginning of combat on your turn, mill a card. When you do,
//! if there are four or more creature cards in your graveyard, put a
//! +1/+1 counter on target creature you control and it gains
//! deathtouch until end of turn."
//!
//! The mill half is faithful. GAP: the reflexive "When you do, if four
//! or more creature cards in your graveyard …" clause needs a
//! type-filtered graveyard count (script only exposes total
//! `graveyard_size`) plus a reflexive targeted trigger; that half is
//! GAPped.

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
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sigil of Myrkul");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: mill_and_reward,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…mill a card. When you do, if there are four or more creature
/// cards in your graveyard, put a +1/+1 counter on target creature you
/// control and it gains deathtouch until end of turn."
fn mill_and_reward(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the reflexive "When you do, if four or more CREATURE cards
    // in your graveyard, put a +1/+1 counter on target creature you
    // control and it gains deathtouch" clause is not expressible — no
    // type-filtered graveyard count helper and no reflexive targeted
    // sub-trigger. Only the mill resolves.
    vec![Effect::Mill {
        player: trig.controller,
        count: 1,
    }]
}
