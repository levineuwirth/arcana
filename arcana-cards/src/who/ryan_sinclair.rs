//! Ryan Sinclair — `{2}{R}` 2/2 Legendary Human.
//! "Whenever Ryan attacks, exile cards from the top of your library until you
//! exile a nonland card. You may cast the exiled card without paying its mana
//! cost if it's a spell with mana value less than or equal to Ryan's power.
//! Put the exiled cards not cast this way on the bottom of your library in a
//! random order."
//! "Doctor's companion."
//!
//! GAP (keyword): "Doctor's companion" is a Commander deck-building keyword
//! with no KeywordAbility variant — omitted.
//! GAP (attack effect): the impulse "exile until you exile a nonland card,
//! then cast it for FREE if its mana value ≤ this creature's power" combines
//! a conditional free cast with a power-gated mana-value check. ImpulseExile
//! plays at normal cost over a fixed count, so this exact shape is
//! inexpressible. The attack trigger is emitted with a GAP'd effect.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Ryan Sinclair");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: impulse_free_cast,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn impulse_free_cast(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: see module doc — power-gated conditional free cast off an
    // exile-until-nonland dig is not expressible with the effect catalog.
    Vec::new()
}
