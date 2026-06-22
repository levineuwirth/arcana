//! Vhal, Scholar of Tactics — `{2}{W}{U}` 4/4 Legendary Creature — Human Wizard.
//! When this creature specializes, remove all study counters from it. When you
//! do, distribute that many +1/+1 counters among any number of target
//! creatures.
//! {T}: Draw a card, then discard a card.
//!
//! The specialize trigger is registered (SelfSpecializes). Its body —
//! "remove all study counters, then distribute that many +1/+1 counters among
//! any number of target creatures" — needs a remove-all-counters amount fed
//! into a reflexive distribute-among-any-number effect, which has no
//! demonstrated primitive, so the body is GAP'd. The {T} loot activated
//! ability (draw a card, then discard a card) is wired.

use arcana_core::effects::{DiscardChoice, Effect};
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vhal, Scholar of Tactics");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfSpecializes,
                intervening_if: None,
                effect: on_specialize,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Draw a card, then discard a card.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: loot,
            }),
    )
}

fn on_specialize(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "remove all study counters from it. When you do, distribute that
    // many +1/+1 counters among any number of target creatures." — needs a
    // remove-all-counters amount fed into a reflexive distribute-among-any-
    // number effect; no demonstrated primitive.
    Vec::new()
}

fn loot(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
