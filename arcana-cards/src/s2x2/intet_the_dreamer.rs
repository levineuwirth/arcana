//! Intet, the Dreamer — `{3}{G}{U}{R}` 6/6 Legendary Dragon with Flying.
//!
//! Oracle:
//! * Flying
//! * Whenever Intet deals combat damage to a player, you may pay {2}{U}.
//!   If you do, exile the top card of your library face down. You may look
//!   at that card for as long as it remains exiled. You may play that card
//!   without paying its mana cost for as long as Intet remains on the
//!   battlefield.
//!
//! The combat-damage trigger + the "you may pay {2}{U}" gate are modeled
//! with `DamageDealt` (to a player, combat) → `OptionalPayment` of {2}{U}.
//!
//! GAP: the "if you do" payload — exile the top card face down and grant
//! permission to play it for free for as long as Intet remains on the
//! battlefield — has no matching primitive (ImpulseExile plays at normal
//! cost and only until end of turn, which is both the wrong cost and the
//! wrong duration). The paid body is left as a no-op pending an
//! exile-and-play-free-while-source-remains effect.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Intet, the Dreamer");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: pay_then_exile,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "you may pay {2}{U}. If you do, [exile top card face down, play it for
/// free while Intet remains on the battlefield]."
fn pay_then_exile(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}{U}").expect("valid cost")),
        // GAP: exile-top-face-down + play-for-free-while-Intet-on-battlefield
        // has no matching primitive; paid body is a no-op.
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: None,
    }]
}
