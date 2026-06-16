//! Basri Ket — `{1}{W}{W}` Legendary Planeswalker — Basri, starting loyalty 3.
//!
//! +1: Put a +1/+1 counter on up to one target creature. It gains indestructible
//!   until end of turn. (`AddCounters` + `GrantKeyword(Indestructible, EndOfTurn)`.)
//! −2: "Whenever one or more nontoken creatures attack this turn, create that
//!   many 1/1 white Soldier tokens that are tapped and attacking." GAP: this is a
//!   turn-long delayed trigger keyed to a variable attacker count — no demonstrated
//!   surface installs a one-shot reflexive trigger from a loyalty effect. Ability
//!   shell declared with the −2 cost; effect GAP'd.
//! −6: emblem ("At the beginning of combat on your turn, create a 1/1 white
//!   Soldier creature token, then put a +1/+1 counter on each creature you
//!   control."). Triggered emblem: StepBegins(Combat, You) → create token + ForEach
//!   over creatures you control.

use arcana_core::effects::{Effect, EmblemDefinition, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Basri Ket");
    let basri = reg.interner_mut().intern("Basri");
    let _soldier = reg.interner_mut().intern("Soldier");
    let _emblem = reg.interner_mut().intern("Basri Ket emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(basri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Put a +1/+1 counter on up to one target creature. It \
                       gains indestructible until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_counter,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Whenever one or more nontoken creatures attack this \
                       turn, create that many 1/1 white Soldier creature tokens \
                       that are tapped and attacking.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"At the beginning of combat \
                       on your turn, create a 1/1 white Soldier creature token, \
                       then put a +1/+1 counter on each creature you \
                       control.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

fn soldier_token(reg: &CardRegistry) -> TokenDefinition {
    let soldier = reg.interner().lookup("Soldier").expect("Soldier interned");
    let mut st = SubtypeSet::default();
    st.0.insert(soldier);
    TokenDefinition {
        name: soldier,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: st,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    }
}

fn plus_one_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn minus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: install a turn-long delayed reflexive trigger ("whenever one or more
    //      nontoken creatures attack this turn, create that many tokens") — no
    //      demonstrated surface installs a one-shot variable-count trigger.
    Vec::new()
}

fn minus_six_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Basri Ket emblem").expect("emblem interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_combat,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_combat(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: soldier_token(reg),
        },
        Effect::ForEach {
            targets: creatures,
            effect: Box::new(Effect::AddCounters {
                target: NULL_OBJECT_ID,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
        },
    ]
}
