//! Famished Worldsire — `{5}{G}{G}{G}` 0/0 Leviathan with Ward {3} and
//! Devour 3.
//! "When this creature enters, look at the top X cards of your library,
//!  where X is this creature's power. Put any number of land cards from
//!  among them onto the battlefield tapped, then shuffle."
//!
//! Ward {3} and Devour 3 are expressible (the "land" Devour restriction is
//! a fidelity gap). The ETB dig is not expressible — DigTopN/RevealUntil
//! are single-take with a fixed count, but this looks at a dynamic X and
//! puts ANY NUMBER of lands onto the battlefield — GAP'd effect, trigger
//! structure kept.

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
    let name = reg.interner_mut().intern("Famished Worldsire");
    let leviathan = reg.interner_mut().intern("Leviathan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leviathan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![
            KeywordAbility::Ward(ManaCost::parse("{3}").expect("valid cost")),
            KeywordAbility::Devour(3),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: dig_for_lands,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dig_for_lands(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at the top X cards (X = power), put ANY NUMBER of land
    // cards onto the battlefield tapped, then shuffle" — DigTopN/RevealUntil
    // are single-take with a fixed count; the variable-X multi-put is not
    // expressible.
    Vec::new()
}
