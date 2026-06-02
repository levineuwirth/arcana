//! Seismic Elemental — `{3}{R}{R}` 4/4 red Elemental. "When this creature
//! enters, creatures without flying can't block this turn."
//!
//! ETB trigger via `TriggerCondition::SelfEntersBattlefield`. The effect
//! forbids blocking this turn on each creature on the battlefield via
//! `Effect::ForbidBlocking` over `script::ids_matching`.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seismic Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: forbid_nonfliers_blocking,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn forbid_nonfliers_blocking(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: ObjectFilter has no "without flying" refinement, so this forbids
    // blocking on ALL creatures rather than only those without flying.
    let ids = script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    ids.into_iter()
        .map(|id| Effect::ForbidBlocking {
            target: id,
            duration: Duration::EndOfTurn,
        })
        .collect()
}
