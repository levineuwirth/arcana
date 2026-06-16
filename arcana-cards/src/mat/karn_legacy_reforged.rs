//! Karn, Legacy Reforged — `{5}` */* Legendary Artifact Creature — Golem.
//!
//! * "Karn's power and toughness are each equal to the greatest mana value among
//!   artifacts you control." — characteristic-defining ability, not expressible
//!   as a continuous P/T; base 0/0 with the CDA GAP'd.
//! * "At the beginning of your upkeep, add {C} for each artifact you control."
//!   The "can't be spent on nonartifact spells" and "don't lose this mana" riders
//!   are not expressible (see GAP); the mana production itself is modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karn, Legacy Reforged");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // GAP: */* CDA (P/T = greatest mana value among artifacts you control); base 0/0.
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: add_mana_per_artifact,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn add_mana_per_artifact(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "can't be spent on nonartifact spells" + "don't lose this mana as steps and
    // phases end" riders are not expressible; the {C}-per-artifact production is modeled.
    let n = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, trig.source); n as usize],
    }]
}
