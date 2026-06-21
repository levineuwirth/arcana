//! Gimbal, Gremlin Prodigy — `{2}{G}{U}{R}` 4/4 Legendary Gremlin
//! Artificer. "Artifact creatures you control have trample" is a static
//! anthem with no primitive — GAP'd. "At the beginning of your end step,
//! create a 0/0 red Gremlin artifact creature token. Put X +1/+1 counters
//! on it, where X is the number of differently named artifact tokens you
//! control." The token creation is wired; the counter rider is GAP'd (no
//! handle on the freshly minted token to counter, and no script helper for
//! "differently named artifact tokens").

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gimbal, Gremlin Prodigy");
    let gremlin = reg.interner_mut().intern("Gremlin");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gremlin);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    // GAP: static "Artifact creatures you control have trample" — no anthem /
    // keyword-grant static primitive in the demonstrated API.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: end_step_make_gremlin,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn end_step_make_gremlin(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let gremlin = reg.interner().lookup("Gremlin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gremlin);
    let token = TokenDefinition {
        name: gremlin,
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "Put X +1/+1 counters on it, where X = differently named artifact
    // tokens you control" — no handle on the freshly minted token to target,
    // and no script helper for "differently named artifact tokens".
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
