//! Deep Spawn — `{5}{U}{U}{U}` 6/6 Creature — Homarid.
//! Trample.
//! At the beginning of your upkeep, sacrifice this creature unless you mill
//! two cards.
//! {U}: This creature gains shroud until end of turn and doesn't untap during
//! your next untap step. Tap this creature.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deep Spawn");
    let homarid = reg.interner_mut().intern("Homarid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(homarid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_sac_unless_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}: This creature gains shroud until end of turn and doesn't untap during your next untap step. Tap this creature."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_shroud_and_tap,
            }),
    )
}

fn upkeep_sac_unless_mill(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "sacrifice this creature unless you mill two cards." The "unless"
    // gate's payment is milling 2 cards — OptionalPaymentKind supports only a
    // fixed Mana cost or a Life amount, not milling-as-cost. Not expressible.
    Vec::new()
}

fn gain_shroud_and_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Grants shroud until end of turn and taps itself. GAP: "doesn't untap
    // during your next untap step" has no documented effect primitive.
    vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Shroud,
            duration: Duration::EndOfTurn,
        },
        Effect::Tap { target: ctx.source },
    ]
}
