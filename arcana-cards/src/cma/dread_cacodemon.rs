//! Dread Cacodemon — `{7}{B}{B}{B}` 8/8 black Demon.
//! "When this creature enters, if you cast it from your hand, destroy all creatures
//! your opponents control, then tap all other creatures you control."
//! GAP: "if cast from your hand" intervening-if condition not accessible via engine;
//! using None for intervening_if.
//! Best-effort: destroy all opponent creatures via ForEach, tap all your other creatures
//! via ForEach.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dread Cacodemon");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: "if you cast it from your hand" not expressible as intervening_if
                intervening_if: None,
                effect: etb_destroy_opponents_tap_yours,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_destroy_opponents_tap_yours(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Destroy all creatures opponents control
    let opp_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        trig.controller,
    );
    let destroy_all = Effect::ForEach {
        targets: opp_creatures,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    };
    // Tap all other creatures you control (excluding this creature itself via best-effort)
    let your_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let tap_all: Vec<Effect> = your_creatures
        .into_iter()
        .filter(|&id| id != trig.source)
        .map(|id| Effect::Tap { target: id })
        .collect();
    let mut result = vec![destroy_all];
    result.push(Effect::Sequence(tap_all));
    result
}
