//! Kagemaro, First to Suffer — `{3}{B}{B}` */* Legendary Demon Spirit.
//! Kagemaro's power and toughness are each equal to the number of cards
//! in your hand (CDA — recorded as */* via PtValue::Star; wired at Layer
//! 7a via `ContinuousEffect::self_pt_cda` on a `SelfEntersBattlefield`
//! trigger).
//! {B}, Sacrifice Kagemaro: All creatures get -X/-X until end of turn,
//! where X is the number of cards in your hand.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kagemaro, First to Suffer");
    let demon = reg.interner_mut().intern("Demon");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
        .with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        })
        .with_activated_ability(ActivatedAbilityDef {
            text: "{B}, Sacrifice Kagemaro: All creatures get -X/-X until end of turn, where X is the number of cards in your hand.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: all_creatures_minus_x,
        }),
    )
}

/// "Kagemaro's power and toughness are each equal to the number of cards
/// in your hand" — install the self-CDA at Layer 7a.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            hand_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power and toughness each equal to cards in your hand.
fn hand_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = script::hand_size(s, who) as i32;
    (n, n)
}

fn all_creatures_minus_x(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::hand_size(state, ctx.controller) as i32;
    let ids = script::ids_matching(state, &ObjectFilter::creature(), ctx.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: -x,
            toughness: -x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
