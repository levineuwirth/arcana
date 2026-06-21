//! Crater Hellion — `{4}{R}{R}` 6/6 red Hellion Beast.
//! "Echo {4}{R}{R}. When this creature enters, it deals 4 damage to
//! each other creature."
//!
//! Abilities:
//! 1. Echo {4}{R}{R} — NOT in the supported KeywordAbility surface; GAP'd.
//! 2. SelfEntersBattlefield → deal 4 damage to each OTHER creature.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Crater Hellion");
    let hellion = reg.interner_mut().intern("Hellion");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hellion);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: Echo {4}{R}{R} is not in the supported KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: damage_each_other_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn damage_each_other_creature(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "each OTHER creature" — every creature on the battlefield except
    // this one.
    let ids: Vec<_> = script::ids_matching(state, &ObjectFilter::creature(), trig.controller)
        .into_iter()
        .filter(|id| *id != trig.source)
        .collect();
    ids.into_iter()
        .map(|id| Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Object(id),
            amount: 4,
        })
        .collect()
}
