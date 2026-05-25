//! Bane of Progress — `{4}{G}{G}` 2/2 green Elemental. "When this creature enters,
//! destroy all artifacts and enchantments. Put a +1/+1 counter on this creature for
//! each permanent destroyed this way."
//! GAP: "count destroyed permanents and add that many counters" not supported atomically;
//! using ForEach over matching permanents to destroy, then separate counter add.
//! GAP: count of destroyed permanents not accessible after the destroy effects;
//! pre-counting artifacts+enchantments and using that count.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bane of Progress");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_destroy_all,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_destroy_all(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT));
    let ids = script::ids_matching(state, &filter, trig.controller);
    let n = ids.len() as u32;
    let mut effects: Vec<Effect> = vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        },
    ];
    if n > 0 {
        effects.push(Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: n,
        });
    }
    effects
}
