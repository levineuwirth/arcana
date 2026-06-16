//! Tom, Bert, and William — `{3}{B}{G}` 5/5 Legendary Troll.
//! "{1}, Sacrifice another creature: Draw cards equal to the
//! sacrificed creature's power, then discard a card." — the draw count
//! is the power of the cost-sacrificed creature, which has no
//! activation-context accessor → that draw is GAP'd; the discard is
//! emitted.
//! "When Tom, Bert, and William die, if they were a creature, return
//! them to the battlefield. They're an artifact." — the return is
//! emitted; the post-return "they're an artifact" type swap rides on a
//! new object id and is GAP'd.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tom, Bert, and William");
    let troll = reg.interner_mut().intern("Troll");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(troll);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Sacrifice another creature: Draw cards equal to the \
                       sacrificed creature's power, then discard a card."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").unwrap(),
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: sac_draw_discard,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: return_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn sac_draw_discard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Draw cards equal to the sacrificed creature's power" — the
    // cost-sacrificed creature's power has no ActivationContext
    // accessor, so the dynamic draw count is unobtainable. The "then
    // discard a card" half is expressed.
    vec![Effect::Discard {
        player: ctx.controller,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}

fn return_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "They're an artifact (no longer a creature)" — the returned
    // permanent receives a fresh object id, so a post-return AddType
    // rider cannot be aimed at it; only the return is expressed.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: trig.source }]
}
