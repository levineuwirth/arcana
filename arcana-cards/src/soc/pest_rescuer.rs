//! Pest Rescuer — `{2}{G}` 2/2 green Dryad Druid.
//! At the beginning of each upkeep, if you don't control a Pest creature token,
//! create a 1/1 black and green Pest creature token with "When this token dies,
//! you gain 1 life."
//! If you would gain life, you gain that much life plus 1 instead.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pest Rescuer");
    let dryad = reg.interner_mut().intern("Dryad");
    let druid = reg.interner_mut().intern("Druid");
    let _pest = reg.interner_mut().intern("Pest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::Any,
            },
            intervening_if: Some(if_no_pest_token),
            effect: make_pest_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
        // GAP: "If you would gain life, you gain that much life plus 1 instead."
        // A life-gain replacement static; no demonstrated replacement primitive.
    )
}

fn if_no_pest_token(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    let filter = script::subtype_filter(reg, "Pest")
        .controlled_by(ControllerConstraint::You)
        .tokens_only();
    arcana_core::conditions::you_control_at_most(s, you, &filter, 0)
}

fn make_pest_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let pest = reg.interner().lookup("Pest").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pest);
    // GAP: the token's printed "When this token dies, you gain 1 life" ability
    // is not attached (token abilities are emitted empty here).
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: pest,
            colors: ColorSet::black() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
