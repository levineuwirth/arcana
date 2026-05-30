//! Flourishing Hunter — `{4}{G}{G}` 6/6 green Wolf Spirit creature.
//! "When this creature enters, you gain life equal to the greatest toughness
//! among other creatures you control."
//! ETB trigger: gain life equal to the max toughness of other creatures you control.

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
    let name = reg.interner_mut().intern("Flourishing Hunter");
    let wolf = reg.interner_mut().intern("Wolf");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_gain_life(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "greatest toughness among OTHER creatures you control" — exclude self.
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    let max_toughness = ids
        .iter()
        .filter(|&&id| id != trig.source)
        .map(|&id| script::toughness_of(state, id))
        .max()
        .unwrap_or(0);
    let amount = max_toughness.max(0) as u32;
    if amount == 0 {
        return Vec::new();
    }
    vec![Effect::GainLife { player: trig.controller, amount }]
}
