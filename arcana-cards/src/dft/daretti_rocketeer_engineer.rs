//! Daretti, Rocketeer Engineer — `{4}{R}` */5 Legendary Goblin Artificer.
//!
//! "Daretti's power is equal to the greatest mana value among artifacts you
//! control." (a `*` power CDA — engine machinery; modeled as PtValue::Star)
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
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
        // GAP (CDA): power "equal to the greatest mana value among artifacts
        // you control" — characteristic-defining ability is engine-side;
        // modeled as the `*` placeholder.
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
            }),
    )
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
