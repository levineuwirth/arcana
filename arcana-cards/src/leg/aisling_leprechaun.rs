//! Aisling Leprechaun — `{G}` 1/1 green Faerie. "Whenever this creature
//! blocks or becomes blocked by a creature, that creature becomes green.
//! (This effect lasts indefinitely.)"
//!
//! Uses `SelfBlocksOrBecomesBlocked` + `trig.other_combatant()` to read
//! "that creature", then `Effect::SetColor` to make it green.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Aisling Leprechaun");
    let faerie = reg.interner_mut().intern("Faerie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
                intervening_if: None,
                effect: make_that_creature_green,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_that_creature_green(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.other_combatant() else { return Vec::new(); };
    // "This effect lasts indefinitely." No indefinite Duration is exposed;
    // WhileSourceOnBattlefield is the closest persistent duration available.
    vec![Effect::SetColor {
        target: id,
        colors: ColorSet::green(),
        duration: Duration::WhileSourceOnBattlefield,
    }]
}
