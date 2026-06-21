//! Lucius the Eternal — `{3}{B}{R}` 5/3 Legendary Astartes Warrior with Haste.
//!
//! Haste.
//! Armour of Shrieking Souls — When Lucius the Eternal dies, exile it and
//! choose target creature an opponent controls. When that creature leaves the
//! battlefield, return this card from exile to the battlefield under its
//! owner's control.
//!
//! GAP: the dies-trigger's "exile self, then return when a DIFFERENT chosen
//! creature leaves the battlefield" linkage is not expressible. The only
//! exile-with-linked-return primitive is `Effect::ExileUntilSourceLeaves`,
//! which links the return to the SOURCE (jailer) leaving, not to a separate
//! chosen creature leaving. There is no primitive to schedule a return keyed
//! on another object's zone change. The trigger condition (SelfDies) fires
//! faithfully; the effect body is the gap.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lucius the Eternal");
    let astartes = reg.interner_mut().intern("Astartes");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(astartes);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: armour_of_shrieking_souls,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// GAP: exile-self-and-return-when-another-chosen-creature-leaves is not
/// expressible (see module doc).
fn armour_of_shrieking_souls(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
