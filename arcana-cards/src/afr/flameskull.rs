//! Flameskull — `{1}{R}{R}` 3/1 Skeleton with Flying.
//!
//! Oracle:
//! * Flying (keyword).
//! * This creature can't block. (static — GAP'd; no permanent "can't block"
//!   primitive in this shape — `ForbidBlocking` is a duration-bounded effect.)
//! * Rejuvenation — When this creature dies, exile it. If you do, exile the
//!   top card of your library. Until the end of your next turn, you may play
//!   one of those cards.
//!
//! The dies trigger is modeled with `Effect::ImpulseExile { count: 1 }`, which
//! exiles the top card of the library and grants play-permission. Fidelity
//! gaps: ImpulseExile grants permission only until end of turn (oracle is
//! "until end of your next turn"), the "exile Flameskull itself and you may
//! also play it" half is not modeled, and the mutual-exclusion rider is
//! omitted.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Flameskull");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);

    // GAP: "This creature can't block." — a permanent static with no expressible
    // primitive (ForbidBlocking is duration-bounded, not a permanent static).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: rejuvenation,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn rejuvenation(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Exile the top card of your library; you may play it (impulse).
    // GAP: "exile it [Flameskull]" half + "until end of your NEXT turn" window
    // + mutual-exclusion rider not modeled.
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: 1,
    }]
}
