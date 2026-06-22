//! Yuna, Hope of Spira — `{3}{G}{W}` 3/5 Legendary Creature — Human Cleric.
//!
//! Oracle:
//! * During your turn, Yuna and enchantment creatures you control have
//!   trample, lifelink, and ward {2}. (Static, turn-gated, board-wide keyword
//!   grant — no demonstrated triggered/activated API expresses a continuous
//!   "during your turn … have keywords" anthem; GAP'd.)
//! * At the beginning of your end step, return up to one target enchantment
//!   card from your graveyard to the battlefield with a finality counter on
//!   it. (Wired as an end-step targeted trigger.)

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
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yuna, Hope of Spira");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    // Pre-intern the named "finality" counter for the resolver lookup.
    let _ = reg.interner_mut().intern("finality");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: "During your turn, Yuna and enchantment creatures you control
        // have trample, lifelink, and ward {2}." — a turn-gated, board-wide
        // continuous keyword anthem; not a triggered/activated ability and not
        // expressible with the demonstrated API.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: reanimate_enchantment,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ENCHANTMENT.into())
                        .controlled_by(ControllerConstraint::You),
                },
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn reanimate_enchantment(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let finality = reg
        .interner()
        .lookup("finality")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::PlusOnePlusOne);
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        Effect::AddCounters {
            target: *id,
            kind: finality,
            count: 1,
        },
    ]
}
