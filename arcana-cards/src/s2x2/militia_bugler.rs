//! Militia Bugler — `{2}{W}` 2/3 white Human Soldier.
//! Vigilance.
//! When this creature enters, look at the top four cards of your library. You
//! may reveal a creature card with power 2 or less from among them and put it
//! into your hand. Put the rest on the bottom of your library in a random order.
//!
//! Abilities:
//!  - Vigilance → `KeywordAbility::Vigilance`.
//!  - ETB → DigTopN(4) with a "creature card with power 2 or less" filter;
//!    chosen card goes to hand, the rest to the bottom in random order.

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Militia Bugler");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_dig_for_small_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_dig_for_small_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature().with_max_power(2);
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 4,
        filter: Some(filter),
        rest: DigRest::BottomRandom,
    }]
}
