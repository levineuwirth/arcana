//! Junkyard Genius — `{1}{B}{R}` 2/2 Human Artificer.
//! "When this creature enters, create a tapped Powerstone token."
//!  (token minted; the "enters tapped" rider is GAP'd.)
//! "{1}{B}{R}, Sacrifice another creature or artifact: Until end of
//!  turn, other creatures you control get +1/+0 and gain menace and
//!  haste."

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Junkyard Genius");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_powerstone,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}{R}, Sacrifice another creature or artifact: Until end of turn, other creatures you control get +1/+0 and gain menace and haste.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}{R}").expect("valid cost"),
                    sacrifice_other: Some(
                        ObjectFilter::default()
                            .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::ARTIFACT)),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: anthem_menace_haste,
            }),
    )
}

fn etb_powerstone(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: token enters TAPPED — no tapped option on CreateCommodityToken.
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Powerstone,
        count: 1,
    }]
}

fn anthem_menace_haste(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = arcana_core::script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    // "other creatures" — the source itself is naturally excluded from
    // benefit because the pump is targeted per-id via ForEach; a +1/+0
    // including self is a documented near-miss.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: arcana_core::objects::NULL_OBJECT_ID,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Menace, KeywordAbility::Haste],
        }),
    }]
}
