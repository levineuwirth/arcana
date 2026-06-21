//! Jet Collector — `{1}{G}` 2/2 Creature — Merfolk Scout.
//!
//! * "At the beginning of your second main phase, if four or more cards are in
//!   your graveyard, conjure a card named Mox Jet into your hand. This ability
//!   triggers only once." — the trigger and its intervening-if (graveyard >= 4)
//!   are wired, but Conjure is an Alchemy/Arena-only mechanic with no Effect
//!   variant, so the effect body is GAP'd (empty).
//! * "{X}{B}: Put target creature card with mana value X from your graveyard
//!   onto the battlefield with a finality counter on it. Activate only as a
//!   sorcery." — reanimates the targeted graveyard creature card with a
//!   finality counter (`ReturnFromGraveyardWithCounters`). The "mana value X"
//!   target restriction (source-relative to the X paid) is not expressible and
//!   is GAP'd, leaving any creature card in the graveyard targetable.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
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
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jet Collector");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let scout = reg.interner_mut().intern("Scout");
    let _finality = reg.interner_mut().intern("finality");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PostCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_four_in_graveyard),
                effect: conjure_mox_jet,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerGame,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{B}: Put target creature card with mana value X from your graveyard onto the battlefield with a finality counter on it. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    // GAP: "with mana value X" — no X-relative filter; any
                    // creature card in the graveyard is targetable.
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reanimate_with_finality,
            }),
    )
}

fn if_four_in_graveyard(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::graveyard_at_least(s, you, 4)
}

fn conjure_mox_jet(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure not modeled (Arena-only mechanic; would need
    // registry-by-name lookup in Effect::execute).
    Vec::new()
}

fn reanimate_with_finality(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let finality = reg
        .interner()
        .lookup("finality")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::PlusOnePlusOne);
    vec![Effect::ReturnFromGraveyardWithCounters {
        target: *id,
        kind: finality,
        count: 1,
    }]
}
