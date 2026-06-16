//! Will Kenrith — `{4}{U}{U}` Legendary Planeswalker — Will, starting loyalty 5.
//!
//! Partner with Rowan Kenrith / Partner — not in the usable keyword
//!   surface, so `keywords: vec![]` (gap noted).
//! +2: Until your next turn, up to two target creatures each have base
//!   power and toughness 0/3 and lose all abilities. Modeled as
//!   `SetBasePT` 0/3 + `LoseAllAbilities`, duration UntilYourNextTurn,
//!   over up to two targets.
//! −2: Target player draws two cards. Until your next turn, instant,
//!   sorcery, and planeswalker spells that player casts cost {2} less.
//!   Modeled as the draw; the cost-reduction rider is GAP'd.
//! −8: Target player gets an emblem with "Whenever you cast an instant
//!   or sorcery spell, copy it. You may choose new targets for the
//!   copy." The emblem is created under the TARGET player's control with
//!   an I/S SpellCast trigger; the "copy it" effect references the
//!   triggering spell's stack entry (not exposed to the trigger effect),
//!   so the copy is GAP'd.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Will Kenrith");
    let will = reg.interner_mut().intern("Will");
    let _emblem = reg.interner_mut().intern("Will Kenrith emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(will);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        // GAP: Partner / Partner with are not in the usable keyword surface.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Until your next turn, up to two target creatures \
                       each have base power and toughness 0/3 and lose all \
                       abilities.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_neuter,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Target player draws two cards. Until your next \
                       turn, instant, sorcery, and planeswalker spells that \
                       player casts cost {2} less to cast.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: Target player gets an emblem with \"Whenever you \
                       cast an instant or sorcery spell, copy it. You may \
                       choose new targets for the copy.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

fn plus_two_neuter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let dur = Duration::UntilYourNextTurn(ctx.controller);
    let mut effects = Vec::new();
    for target in &ctx.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::SetBasePT {
                target: *id,
                power: 0,
                toughness: 3,
                duration: dur.clone(),
            });
            effects.push(Effect::LoseAllAbilities {
                target: *id,
                duration: dur.clone(),
            });
        }
    }
    effects
}

fn minus_two_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "I/S/PW spells that player casts cost {2} less until your next
    // turn" cost-reduction rider isn't expressible; the draw is.
    vec![Effect::DrawCards { player: *p, count: 2 }]
}

fn minus_eight_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = _ctx.targets.targets.first() else {
        return Vec::new();
    };
    let emblem_name = reg
        .interner()
        .lookup("Will Kenrith emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        // "Target player gets an emblem" — the emblem is controlled by
        // the targeted player.
        controller: *p,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        types_any: Some(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                        ..Default::default()
                    }),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_copy,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "copy it" references the triggering spell's stack entry, which
    // isn't exposed to the trigger effect.
    Vec::new()
}
