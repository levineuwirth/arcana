//! Daretti, Rocketeer Engineer — `{4}{R}` */5 Legendary Goblin Artificer.
//!
//! "Daretti's power is equal to the greatest mana value among artifacts you
//! control." (a `*` power CDA — wired via a self-CDA installed on ETB,
//! ContinuousEffect::self_pt_cda at Layer 7a; only POWER is `*`, so the scalar
//! compute returns the greatest mana value among artifacts you control for
//! power and the printed fixed `5` toughness.)
//! "Whenever Daretti enters or attacks, choose target artifact card in your
//! graveyard. You may sacrifice an artifact. If you do, return the chosen
//! card to the battlefield."
//!
//! The enters/attacks pair becomes two triggered abilities, each targeting
//! an artifact card in your graveyard and returning it to the battlefield.
//! The "you may sacrifice an artifact" payment gate is GAP'd (OptionalPayment
//! supports only Mana/Life, not a sacrifice cost), so the return is applied
//! unconditionally — a documented fidelity gap.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
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
    let name = reg.interner_mut().intern("Daretti, Rocketeer Engineer");
    let goblin = reg.interner_mut().intern("Goblin");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    fn artifact_card_target() -> TargetRequirement {
        TargetRequirement {
            filter: TargetFilter::Card {
                zone: Zone::Graveyard(0),
                filter: ObjectFilter::new()
                    .with_types(TypeLine::ARTIFACT.into())
                    .controlled_by(ControllerConstraint::You),
            },
            count: TargetCount::Exactly(1),
            controller: None,
        }
    }

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: reanimate_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![artifact_card_target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: reanimate_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![artifact_card_target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Layer 7a self-CDA. Only POWER is `*` (= the greatest mana value among
/// artifacts you control); toughness is the printed fixed `5`.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = greatest mana value among artifacts you control (0 if none);
/// toughness = printed 5.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let greatest = s
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == who && o.characteristics.types.is_artifact())
        .map(|o| {
            o.characteristics
                .mana_cost
                .as_ref()
                .map(|c| c.mana_value())
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0) as i32;
    (greatest, 5)
}

fn reanimate_artifact(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP (cost gate): "You may sacrifice an artifact. If you do, …" —
    // OptionalPayment supports only Mana/Life, not a sacrifice payment, so
    // the return is applied unconditionally.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
