//! Vincent, Vengeful Atoner — `{2}{R}` 3/3 Legendary Assassin with Menace.
//! "Whenever one or more creatures you control deal combat damage to a
//! player, put a +1/+1 counter on Vincent."
//! Chaos — "Whenever Vincent deals combat damage to an opponent, it deals
//! that much damage to each other opponent if Vincent's power is 7 or
//! greater." (GAP.)
//!
//! Menace is a base keyword. The first trigger fires on combat damage to
//! a player by a creature you control and adds a +1/+1 counter to Vincent
//! (the "one or more ... a counter" once-per-combat batching is a
//! fidelity GAP — DamageDealt fires per damage event, so multiple
//! attackers may add multiple counters). The Chaos trigger is GAP'd:
//! DamageDealt's source_filter cannot pin the source to Vincent by
//! identity, and the "Vincent's power is 7 or greater" gate has no
//! documented intervening-if condition helper.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vincent, Vengeful Atoner");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: Chaos trigger — DamageDealt source_filter can't pin the source
    // to Vincent by identity, and "Vincent's power is 7 or greater" has no
    // documented intervening-if condition helper.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: counter_on_vincent,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn counter_on_vincent(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
