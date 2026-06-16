//! Chandra, Dressed to Kill — `{1}{R}{R}` Legendary Planeswalker — Chandra, starting loyalty 4.
//!
//! +1: Add {R}. Chandra deals 1 damage to up to one target player or
//!   planeswalker. Modeled as `AddMana({R})` + `DealDamage` to a target
//!   player (the "player or planeswalker" disjunction has no dedicated
//!   target filter; player-targeting is the faithful approximation).
//! +1: Exile the top card of your library; if it's red you may cast it.
//!   The conditional cast-from-exile rider isn't expressible, so this is
//!   modeled as `ImpulseExile(1)` (exiles the top card).
//! −7: Exile the top five cards; cast red spells from among them; get an
//!   emblem ("Whenever you cast a red spell, this emblem deals X damage
//!   to any target, where X is the mana spent"). The exile-and-cast is
//!   approximated with `ImpulseExile(5)`; the emblem is created with the
//!   red-spell trigger, but its damage is dynamic-X (mana spent) so the
//!   effect is GAP'd.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Dressed to Kill");
    let chandra = reg.interner_mut().intern("Chandra");
    let _emblem = reg.interner_mut().intern("Chandra, Dressed to Kill emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add {R}. Chandra deals 1 damage to up to one \
                       target player or planeswalker.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_burn,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Exile the top card of your library. If it's red, \
                       you may cast it this turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_impulse,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Exile the top five cards of your library. You may \
                       cast red spells from among them this turn. You get an \
                       emblem with \"Whenever you cast a red spell, this \
                       emblem deals X damage to any target, where X is the \
                       amount of mana spent to cast that spell.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

fn plus_one_burn(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }];
    if let Some(arcana_core::targets::TargetChoice::Player(p)) =
        ctx.targets.targets.first()
    {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(*p),
            amount: 1,
        });
    }
    effects
}

fn plus_one_impulse(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // The "cast it only if it's red" conditional rider isn't expressible;
    // model the impulse exile of the top card.
    vec![Effect::ImpulseExile { player: ctx.controller, count: 1 }]
}

fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Chandra, Dressed to Kill emblem")
        .expect("emblem name interned");
    vec![
        // "Exile the top five cards; you may cast red spells among them."
        // Modeled as the impulse exile of five cards.
        Effect::ImpulseExile { player: ctx.controller, count: 5 },
        Effect::CreateEmblem {
            controller: ctx.controller,
            emblem: EmblemDefinition {
                name: emblem_name,
                statics: Vec::new(),
                abilities: vec![TriggeredAbilityDef {
                    id: 1,
                    trigger_condition: TriggerCondition::SpellCast {
                        filter: Some(ObjectFilter {
                            colors: Some(ColorSet::red()),
                            ..Default::default()
                        }),
                        caster: ControllerConstraint::You,
                    },
                    intervening_if: None,
                    effect: emblem_dynamic_burn,
                    trigger_zones: vec![Zone::Command],
                    frequency: TriggerFrequency::EachTime,
                    target_requirements: vec![TargetRequirement::any_target()],
                }],
            },
        },
    ]
}

fn emblem_dynamic_burn(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: damage equal to the mana spent on the triggering spell is a
    // dynamic-X amount not exposed to the trigger effect.
    Vec::new()
}
