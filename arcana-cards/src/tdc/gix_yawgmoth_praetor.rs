//! Gix, Yawgmoth Praetor — `{1}{B}{B}` Legendary 3/3 Phyrexian Praetor.
//!
//! Oracle:
//! * Whenever a creature deals combat damage to one of your opponents, its
//!   controller may pay 1 life. If they do, they draw a card.
//!   - Wired (2-player faithful): a DamageDealt{creature → player, combat}
//!     trigger that prompts the may-pay-1-life-then-draw. In 2-player "one
//!     of your opponents" is damaged by your creature, so "its controller"
//!     is you (trig.controller).
//!   - GAP: in multiplayer "its controller" may be any creature's
//!     controller; there is no accessor for the damaging object / its
//!     controller, so the chooser is approximated as trig.controller, and
//!     the "to one of your opponents" target restriction is not expressible
//!     on the trigger (fires on combat damage to any player).
//! * {4}{B}{B}{B}, Discard X cards: Exile the top X cards of target
//!   opponent's library; play them for free. GAP: variable "Discard X"
//!   cost is not expressible (discard_other_count is fixed), and impulse
//!   exile targets only your own library — no opponent-library free-play
//!   primitive.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gix, Yawgmoth Praetor");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let praetor = reg.interner_mut().intern("Praetor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(praetor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: may_pay_life_draw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn may_pay_life_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Life(1),
        then: Box::new(Effect::DrawCards {
            player: trig.controller,
            count: 1,
        }),
        else_effect: None,
    }]
}
