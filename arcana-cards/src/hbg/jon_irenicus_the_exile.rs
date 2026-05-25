//! Jon Irenicus, the Exile — `{2}{U}{B}` 3/5 blue-black Legendary Creature — Elf Wizard.
//! "At the beginning of your end step, draw a card if your library has more cards in it
//! than target opponent's library. Otherwise, each opponent mills five cards."
//! Keywords (Scryfall-parsed): Mill (handled via trigger)
//!
//! # Notes
//! GAP: "draw a card if your library has more cards than target opponent's library" —
//! intervening-if condition comparing library sizes requires script helpers but no
//! Conditional Effect variant with a scriptable predicate; using script to compute inline.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jon Irenicus, the Exile");
    let elf = reg.interner_mut().intern("Elf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_library_compare,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn end_step_library_compare(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(opp) = target else { return Vec::new(); };
    let my_lib = script::library_size(state, trig.controller);
    let opp_lib = script::library_size(state, *opp);
    if my_lib > opp_lib {
        vec![Effect::DrawCards { player: trig.controller, count: 1 }]
    } else {
        script::opponents(state, trig.controller)
            .into_iter()
            .map(|p| Effect::Mill { player: p, count: 5 })
            .collect()
    }
}
