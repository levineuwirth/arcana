//! Chicken Egg — `{1}{R}` 0/1 red Egg.
//! "At the beginning of your upkeep, roll a six-sided die. If you roll a 6,
//! sacrifice this creature and create a 4/4 red Giant Bird creature token."
//!
//! The upkeep trigger uses a die-roll mechanic. The engine does not have a
//! six-sided die effect (only `Effect::FlipCoin`). We model the 1-in-6 chance
//! by treating it as a coin flip (1/6 ≈ not a fair coin), which is the closest
//! available primitive. The actual "roll a d6 and check for 6" is a GAP.
//!
//! # GAP
//! "Roll a six-sided die; if you roll a 6" — no die-roll effect in catalog.
//! Using FlipCoin as closest approximation; verify will flag the d6 gap.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chicken Egg");
    let egg = reg.interner_mut().intern("Egg");
    let _giant = reg.interner_mut().intern("Giant");
    let _bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(egg);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_hatch,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_hatch(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "roll a six-sided die; if you roll a 6" — no d6 roll effect in
    // catalog. Using FlipCoin as closest approximation (semantically wrong —
    // correct probability is 1/6, not 1/2). Verify will flag this.
    let giant = reg.interner().lookup("Giant").expect("Giant interned during register()");
    let bird = reg.interner().lookup("Bird").expect("Bird interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(giant);
    token_subtypes.0.insert(bird);
    let token = TokenDefinition {
        name: bird,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::FlipCoin {
        player: trig.controller,
        win: Box::new(Effect::Sequence(vec![
            Effect::Sacrifice { player: trig.controller, filter: arcana_core::targets::ObjectFilter::creature(), count: 1 },
            Effect::CreateToken { controller: trig.controller, token },
        ])),
        lose: None,
    }]
}
