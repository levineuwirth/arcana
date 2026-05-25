//! Endangered Armodon — `{2}{G}{G}` 4/5 green Elephant.
//! "When you control a creature with toughness 2 or less, sacrifice this creature."
//!
//! GAP: no "state-based / while-condition" trigger in the catalog. Using ZoneChange
//! (creature entering your battlefield with toughness ≤ 2) as the closest approximation,
//! but the self-sacrifice effect is also a GAP (Sacrifice targets a player + filter, not self).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Endangered Armodon");
    let elephant = reg.interner_mut().intern("Elephant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_max_toughness(2),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_small_creature_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_small_creature_enters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Sacrifice self. The Sacrifice effect targets a player + filter; sacrifice this permanent directly.
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        count: 1,
    }]
}
