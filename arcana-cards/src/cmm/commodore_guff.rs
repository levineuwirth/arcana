//! Commodore Guff — `{1}{U}{R}{W}` Legendary Planeswalker — Guff, loyalty 5.
//!
//! Triggered: At the beginning of your end step, put a loyalty counter on
//!   another target planeswalker you control.
//! +1: Create a 1/1 red Wizard creature token with "{T}: Add {R}. Spend this
//!   mana only to cast a planeswalker spell."
//! −3: You draw X cards and Commodore Guff deals X damage to each opponent,
//!   where X is the number of planeswalkers you control.
//!
//! # Rules references
//! * CR 606 — loyalty abilities; CR 113.3c — enters with printed loyalty.
//!
//! # Scope
//! GAP: the Wizard token's mana ability ("{T}: Add {R}. Spend only to cast a
//!   planeswalker spell") is a restricted mana ability not expressible in the
//!   token surface (TokenDefinition has no activated-mana abilities). The
//!   token is minted as a plain 1/1 red Wizard.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::effects::TokenDefinition;
use arcana_core::targets::TargetChoice;
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Commodore Guff");
    let guff = reg.interner_mut().intern("Guff");
    let _wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(guff);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_loyalty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::PLANESWALKER.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::You),
                }],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 red Wizard creature token with \"{T}: Add {R}. Spend this mana only to cast a planeswalker spell.\"".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_wizard,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: You draw X cards and Commodore Guff deals X damage to each opponent, where X is the number of planeswalkers you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_draw_damage,
            }),
    )
}

fn end_step_loyalty(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Loyalty,
        count: 1,
    }]
}

fn plus_one_wizard(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the token's restricted mana ability ("{T}: Add {R}, spend only on
    // a planeswalker spell") is not expressible — minting a plain 1/1 red Wizard.
    let wizard = reg.interner().lookup("Wizard").expect("Wizard interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wizard);
    let token = TokenDefinition {
        name: wizard,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_three_draw_damage(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::PLANESWALKER.into())
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let mut effects = vec![Effect::DrawCards { player: ctx.controller, count: x }];
    for opp in script::opponents(state, ctx.controller) {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(opp),
            amount: x,
        });
    }
    effects
}
