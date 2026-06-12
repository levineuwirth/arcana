//! Phytotitan — `{4}{G}{G}` 7/2 Plant Elemental.
//! "When this creature dies, return it to the battlefield tapped under its owner's control at the beginning of their next upkeep."
//! The return is scheduled via `ScheduleDelayedEffect` at the next
//! upkeep. The death re-ids the card, so the dies-trigger's id is the
//! OLD battlefield id; the fresh graveyard id is recovered by name from
//! the controller's graveyard at trigger resolution.
//! GAP: "returns ... tapped" — no tapped variant of
//! ReturnFromGraveyardToBattlefield; it returns untapped.
//! GAP (narrow): printed timing is the OWNER'S next upkeep;
//! DelayedWhen::NextUpkeep fires at the next upkeep that begins,
//! whoever's turn it is.

use arcana_core::effects::{DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phytotitan");
    let plant = reg.interner_mut().intern("Plant");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_dies(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // The death re-ids the card: `trig.dying_object()` is the OLD
    // battlefield id. Recover the fresh graveyard id by name (highest
    // id = most recent arrival), then schedule the return for the
    // beginning of the next upkeep.
    let Some(name) = reg.interner().lookup("Phytotitan") else {
        return Vec::new();
    };
    let Some(gy_id) = state
        .objects
        .objects_in_zone(Zone::Graveyard(trig.controller))
        .filter(|o| o.characteristics.name == name)
        .map(|o| o.id)
        .max()
    else {
        return Vec::new();
    };
    vec![Effect::ScheduleDelayedEffect {
        source: gy_id,
        controller: trig.controller,
        when: DelayedWhen::NextUpkeep,
        effect: delayed_return,
    }]
}

/// "Return it to the battlefield ... at the beginning of their next
/// upkeep." `pt.source` is the graveyard id captured at death; the
/// effect no-ops if the card has since left the graveyard.
/// GAP: returns untapped (no tapped return-from-graveyard variant).
fn delayed_return(
    _state: &GameState,
    pt: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToBattlefield { target: pt.source }]
}
