//! Saprazzan Bailiff — `{3}{U}{U}` 2/2 Merfolk.
//! When this creature enters, exile all artifact and enchantment cards
//! from all graveyards.
//! When this creature leaves the battlefield, return all artifact and
//! enchantment cards from all graveyards to their owners' hands.
//!
//! Both effects act on cards across ALL graveyards — there is no
//! board-wide graveyard-exile / graveyard-return Effect (script helpers
//! enumerate battlefield permanents only, and there is no all-graveyards
//! mass primitive). GAP both effect bodies; the ETB trigger is wired, and
//! the leaves-the-battlefield trigger is approximated by SelfDies (no
//! dedicated "leaves the battlefield" trigger variant — GAP).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Saprazzan Bailiff");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
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
                effect: etb_exile_graveyards,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: trigger — "leaves the battlefield" approximated by SelfDies.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: ltb_return_graveyards,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_exile_graveyards(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile all artifact/enchantment cards from all graveyards — no
    // board-wide all-graveyards exile primitive.
    Vec::new()
}

fn ltb_return_graveyards(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: return all artifact/enchantment cards from all graveyards to
    // owners' hands — no board-wide all-graveyards return primitive.
    Vec::new()
}
