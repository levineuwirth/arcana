//! Phyrexian Dreadnought — `{1}` 12/12 Artifact Creature — Phyrexian Dreadnought.
//!
//! Trample
//! * When this creature enters, sacrifice it unless you sacrifice any number
//!   of creatures with total power 12 or greater.
//!
//! GAP: the ETB "sacrifice it unless you sacrifice any number of creatures
//! with total power 12 or greater" has no expressible primitive — there is no
//! variable-count sacrifice gated on aggregate total power; the trigger's
//! effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Phyrexian Dreadnought");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let dreadnought = reg.interner_mut().intern("Dreadnought");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(dreadnought);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(12)),
        toughness: Some(PtValue::Fixed(12)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_conditional_sacrifice_gap,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_conditional_sacrifice_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: variable-count sacrifice gated on aggregate total power ≥ 12 is
    // not expressible.
    Vec::new()
}
