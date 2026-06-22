//! Rat King, Verminister — `{1}{B}` 1/1 legendary Rat Avatar (B).
//!
//! * Disappear — At the beginning of your end step, if a permanent
//!   left the battlefield under your control this turn, create a 1/1
//!   black Rat creature token and put a +1/+1 counter on Rat King.
//!   (The intervening-if "a permanent left the battlefield this turn"
//!   has no available predicate — GAP'd to `None`; the create/counter
//!   body is wired.)
//! * {T}, Sacrifice three Rats: Return target creature card and all
//!   other cards with the same name as that card from your graveyard
//!   to the battlefield tapped. (Partial: the single targeted card is
//!   reanimated; the "all other cards with the same name" fan-out and
//!   the "tapped" rider are not expressible — GAP.)
//!
//! "Disappear" is not a usable `KeywordAbility` variant — keyword line
//! is empty.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rat King, Verminister");
    let rat = reg.interner_mut().intern("Rat");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let rat_sac_filter = script::subtype_filter(reg, "Rat").controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening-if "a permanent left the battlefield
                // under your control this turn" has no predicate helper.
                intervening_if: None,
                effect: end_step_breed,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice three Rats: Return target creature card and all other cards with the same name as that card from your graveyard to the battlefield tapped.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(rat_sac_filter),
                    sacrifice_other_count: 3,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reanimate_target,
            }),
    )
}

fn rat_token(reg: &CardRegistry) -> TokenDefinition {
    let mut st = SubtypeSet::default();
    if let Some(r) = reg.interner().lookup("Rat") {
        st.0.insert(r);
    }
    let name = reg.interner().lookup("Rat").unwrap_or_default();
    TokenDefinition {
        name,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: st,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    }
}

fn end_step_breed(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::CreateToken { controller: trig.controller, token: rat_token(reg) },
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}

fn reanimate_target(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "all other cards with the same name" fan-out and the
    // "tapped" rider are not expressible; reanimate the single target.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
