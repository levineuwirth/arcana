//! Discerning Financier — `{2}{W}` 2/3 white Human Noble.
//! At the beginning of your upkeep, if an opponent controls more lands than
//! you, create a Treasure token.
//! {2}{W}: Choose another player. That player gains control of target Treasure
//! you control. You draw a card.
//!
//! Abilities:
//!  - Upkeep trigger: creates a Treasure (CreateCommodityToken). The
//!    intervening-if ("if an opponent controls more lands than you") is a
//!    cross-player comparison not expressible with the available conditions::
//!    helpers → the gate is GAP'd and the trigger fires unconditionally on the
//!    token-creation side (the body is faithful; only the gate is dropped).
//!  - Activated {2}{W}: gains control of a target Treasure (to a chosen player)
//!    + draw. The "choose another player; that player gains control" half is a
//!    ChoosePlayer-then-change-control combination not expressible (ChangeControl
//!    targets a permanent id and a fixed new_controller, and ChoosePlayerThen's
//!    inner effect substitutes a PLAYER field, not a control recipient) → the
//!    control-transfer half is GAP'd; the "You draw a card" half is emitted.

use arcana_core::effects::{CommodityToken, Effect};
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
    let name = reg.interner_mut().intern("Discerning Financier");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP (intervening-if): "if an opponent controls more lands than you"
            // is a cross-player land-count comparison; no conditions:: helper
            // expresses it. Gate dropped; trigger fires unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_make_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}: Choose another player. That player gains control of \
                       target Treasure you control. You draw a card."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                // GAP: the "choose another player; that player gains control of
                // target Treasure" half is not expressible (ChangeControl uses a
                // fixed new_controller; ChoosePlayerThen substitutes a player
                // field, not a control recipient). The targeted-Treasure +
                // player-choice gift is dropped; only "You draw a card" is kept.
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: financier_draw,
            }),
    )
}

fn upkeep_make_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}

fn financier_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}
