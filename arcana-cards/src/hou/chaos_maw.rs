//! Chaos Maw — `{5}{R}{R}` 6/6 red Hellion creature. "When this
//! creature enters, it deals 3 damage to each other creature." A
//! single ETB-trigger that enumerates every other creature on the
//! battlefield and deals 3 damage to each, with Chaos Maw itself as
//! the damage source.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chaos Maw");
    let hellion = reg.interner_mut().intern("Hellion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hellion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
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
                effect: etb_damage_each_other_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: enumerate every creature on the battlefield, drop
/// the Maw itself, and queue 3 damage from the Maw to each of the
/// remaining creatures.
fn etb_damage_each_other_creature(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature(),
        trig.controller,
    );
    let damages: Vec<Effect> = ids
        .into_iter()
        .filter(|id| *id != trig.source)
        .map(|id| Effect::DealDamage {
            target: DamageTarget::Object(id),
            amount: 3,
            source: trig.source,
        })
        .collect();
    vec![Effect::Sequence(damages)]
}
