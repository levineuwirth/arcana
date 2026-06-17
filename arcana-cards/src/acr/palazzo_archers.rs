//! Palazzo Archers — `{2}{G}` 2/3 Human Archer with Reach.
//! "Whenever a creature with flying attacks you or a planeswalker you control,
//!  this creature deals damage equal to its power to that creature."
//! The "attacks you or a planeswalker you control" defending-side restriction
//! isn't expressible on CreatureAttacks's filter — GAP'd; the flying filter and
//! the power-based damage to the attacker are modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Palazzo Archers");
    let human = reg.interner_mut().intern("Human");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "attacks you or a planeswalker you control" defender
            // restriction not expressible; fires on any flying attacker.
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
            },
            intervening_if: None,
            effect: shoot_attacker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn shoot_attacker(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(attacker) = trig.attacking_creature() else {
        return Vec::new();
    };
    let n = script::power_of(state, trig.source).max(0) as u32;
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Object(attacker),
        amount: n,
    }]
}
