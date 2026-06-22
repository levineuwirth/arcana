//! Really Charming Prince — `{1}{U}` 2/1 Faerie Noble with Flying.
//! "When Really Charming Prince enters the battlefield, choose one of the
//! following four cards at random: Piracy Charm, Sapphire Charm, Trickery
//! Charm, and Vision Charm. You may create a copy of the chosen card and cast
//! that copy without paying its mana cost."
//!
//! GAP (ETB effect): randomly choosing one of four specifically-named cards
//! and creating + free-casting a copy of it is a Conjure-class effect over
//! cards not present in any zone — there is no Effect variant for minting a
//! copy of a named card by name (CopyPermanent/CopySpell require an existing
//! object). The trigger is emitted with a GAP'd effect; Flying is expressed.

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
    let name = reg.interner_mut().intern("Really Charming Prince");
    let faerie = reg.interner_mut().intern("Faerie");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_random_charm,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_random_charm(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: see module doc — random selection + free cast of a copy of one of
    // four named cards (Conjure-class) is not expressible.
    Vec::new()
}
