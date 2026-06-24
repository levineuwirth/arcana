//! Veteran Warleader — `{1}{G}{W}` */* Human Soldier Ally.
//! Veteran Warleader's power and toughness are each equal to the number of
//! creatures you control.
//! Tap another untapped Ally you control: This creature gains your choice
//! of first strike, vigilance, or trample until end of turn.
//!
//! The characteristic-defining "power and toughness equal to the number of
//! creatures you control" is installed at Layer 7a via an ETB self-CDA.
//! GAP: "your choice of first strike, vigilance, or trample" is a modal
//! keyword grant — no modal keyword-choice API exists. The activated
//! ability grants FirstStrike as a best-effort stub.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Veteran Warleader");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    subtypes.0.insert(ally);

    let ally_filter = script::subtype_filter(reg, "Ally");

    // CDA "power and toughness equal to the number of creatures you control"
    // is installed at Layer 7a via the ETB self-CDA below.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
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
            text: "Tap another untapped Ally you control: This creature gains \
                   your choice of first strike, vigilance, or trample until end \
                   of turn."
                .into(),
            cost: ActivationCost {
                tap_other: Some(ally_filter),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: grant_keyword_choice,
        }),
    )
}

fn install_cda(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter =
        ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match(
            trig.source,
            filter,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn grant_keyword_choice(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "your choice of first strike, vigilance, or trample" — no modal
    // keyword-choice API. Granting FirstStrike as a best-effort stub.
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::FirstStrike,
        duration: Duration::EndOfTurn,
    }]
}
