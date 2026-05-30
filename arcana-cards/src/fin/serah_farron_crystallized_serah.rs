//! Serah Farron // Crystallized Serah
//!
//! Front: Legendary Creature — Human Citizen {1}{G}{W}, 2/2
//!   The first legendary creature spell you cast each turn costs {2} less.
//!   At the beginning of combat on your turn, if you control 2+ other legendary creatures, you may transform Serah Farron.
//!
//! Back: Legendary Artifact (Crystallized Serah)
//!   The first legendary creature spell you cast each turn costs {2} less.
//!   Legendary creatures you control get +2/+2.
//!
//! GAP: cost-reduction static ability ("first legendary creature spell costs {2} less") not modeled
//! GAP: +2/+2 anthem to legendary creatures not modeled (static layer effect not expressible)
//! GAP: "at the beginning of combat if you control 2+ other legendary creatures" — intervening-if
//!       condition check on legendary count not expressible; wired as BeginningOfCombat trigger
//!       without the intervening-if count guard.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serah Farron");
    let back_name = reg.interner_mut().intern("Crystallized Serah");

    let mut front_subtypes = SubtypeSet::default();
    let human = reg.interner_mut().intern("Human");
    let citizen = reg.interner_mut().intern("Citizen");
    front_subtypes.0.insert(human);
    front_subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger should only fire if you control 2+ other legendary creatures (intervening-if not modeled)
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: transform_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn transform_trigger(
    _state: &arcana_core::state::GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
