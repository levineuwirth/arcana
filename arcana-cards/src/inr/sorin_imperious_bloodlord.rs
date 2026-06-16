//! Sorin, Imperious Bloodlord — `{2}{B}` legendary planeswalker, starting
//! loyalty 4. Subtype Sorin; mono-black.
//!
//! Loyalty abilities:
//! * `+1`: Target creature you control gains deathtouch and lifelink until
//!   end of turn; if it's a Vampire, put a +1/+1 counter on it. (Functional
//!   for the two keyword grants; the Vampire-conditional counter is omitted
//!   — there's no "if target is subtype" conditional in the demonstrated
//!   surface.)
//! * `+1`: You may sacrifice a Vampire; if you do, deal 3 to any target and
//!   gain 3 life. GAP — optional Sacrifice as a gating cost isn't
//!   expressible (`OptionalPayment` supports only mana/life).
//! * `−3`: You may put a Vampire creature card from your hand onto the
//!   battlefield. (Functional via `Effect::PutFromHandOntoBattlefield`.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetFilter, TargetRequirement,
    TargetCount,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sorin, Imperious Bloodlord");
    let sorin = reg.interner_mut().intern("Sorin");
    let _vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sorin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Target creature you control gains deathtouch and \
                       lifelink until end of turn. If it's a Vampire, put a \
                       +1/+1 counter on it."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_grant,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You may sacrifice a Vampire. When you do, Sorin \
                       deals 3 damage to any target and you gain 3 life."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_sac_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: You may put a Vampire creature card from your hand \
                       onto the battlefield."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_put,
            }),
    )
}

fn plus_one_grant(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // Functional: deathtouch + lifelink until end of turn. The Vampire-
    // conditional +1/+1 counter is omitted (no "if target is subtype"
    // conditional in the demonstrated surface).
    vec![
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Deathtouch,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Lifelink,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn plus_one_sac_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: optional Sacrifice as a gating cost is not expressible
    // (OptionalPayment supports only mana / life).
    Vec::new()
}

fn minus_three_put(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let vampire = reg.interner().lookup("Vampire").expect("Vampire interned during register()");
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature().with_subtype_sym(vampire),
        tapped: false,
    }]
}
