//! Exhumer Thrull — `{5}{B}` 3/3 Thrull.
//! Haunt.
//! When this creature enters or the creature it haunts dies, return target
//! creature card from your graveyard to your hand.
//!
//! Haunt is not an emittable KeywordAbility and its "exile haunting target
//! creature on death" mechanic is unmodeled — GAP'd.
//! The compound trigger's ENTERS half is wired (return a target creature
//! card from your graveyard to hand). The "creature it haunts dies" half
//! depends on the (unmodeled) haunt linkage and is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Exhumer Thrull");
    let thrull = reg.interner_mut().intern("Thrull");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thrull);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: Haunt keyword — not an emittable KeywordAbility; the
    // exile-haunting-target-creature-on-death mechanic is unmodeled.
    // GAP: the "or the creature it haunts dies" half of the trigger depends
    // on the unmodeled haunt linkage.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_return_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn etb_return_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
