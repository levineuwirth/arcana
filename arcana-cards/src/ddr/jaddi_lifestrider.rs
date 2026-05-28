//! Jaddi Lifestrider — `{4}{G}` 2/8 green Elemental.
//! "When this creature enters, you may tap any number of untapped creatures you control. You gain 2 life for each creature tapped this way."
//! GAP: "choose any number of untapped creatures to tap; gain 2 per" — multi-creature tap-and-gain not fully expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
    let name = reg.interner_mut().intern("Jaddi Lifestrider");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: lifestrider_effect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn lifestrider_effect(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Approximate: tap all untapped creatures you control, gain 2 per
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .untapped_only();
    let n = script::count_matching(state, &filter, trig.controller);
    let ids = script::ids_matching(state, &filter, trig.controller);
    let mut effects: Vec<Effect> = ids.into_iter().map(|id| Effect::Tap { target: id }).collect();
    if n > 0 {
        effects.push(Effect::GainLife { player: trig.controller, amount: n * 2 });
    }
    effects
}
