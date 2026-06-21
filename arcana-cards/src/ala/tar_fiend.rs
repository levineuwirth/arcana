//! Tar Fiend — `{5}{B}` 4/4 Elemental with Devour 2.
//!
//! Devour 2
//! When this creature enters, target player discards a card for each creature
//! it devoured.
//!
//! Devour 2 is wired as a keyword (the engine handles the sacrifice-and-grow
//! enters-with). The ETB trigger targets a player, but the discard count
//! "for each creature it devoured" needs the number of creatures sacrificed
//! to Devour — there is no `PendingTrigger` accessor for that count, so the
//! dynamic-count effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tar Fiend");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Devour(2)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_discard_per_devoured,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn etb_discard_per_devoured(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "discards a card for each creature it devoured" — no accessor for
    // the number of creatures sacrificed to Devour; dynamic count not
    // expressible.
    Vec::new()
}
