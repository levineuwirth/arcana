//! Sauron, the Lidless Eye — `{3}{B}{R}` 4/4 Legendary Avatar Horror (B/R).
//! "When Sauron enters, gain control of target creature an opponent controls
//! until end of turn. Untap it. It gains haste until end of turn."
//! `{1}{B}{R}: Creatures you control get +2/+0 until end of turn. Each
//! opponent loses 2 life.`
//!
//! The ETB is the Threaten suite (temporary control + untap + haste). The
//! activated ability pumps each creature you control and drains each opponent.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sauron, the Lidless Eye");
    let avatar = reg.interner_mut().intern("Avatar");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
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
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_steal_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}{R}: Creatures you control get +2/+0 until end of \
                       turn. Each opponent loses 2 life."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_team_drain_opponents,
            }),
    )
}

fn etb_steal_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Sequence(vec![
        Effect::ChangeControlEot { target: *id, new_controller: trig.controller },
        Effect::Untap { target: *id },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ])]
}

fn pump_team_drain_opponents(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let team = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let mut effects: Vec<Effect> = team
        .into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: 2,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        })
        .collect();
    for opp in script::opponents(state, ctx.controller) {
        effects.push(Effect::LoseLife { player: opp, amount: 2 });
    }
    vec![Effect::Sequence(effects)]
}
