//! Sower of Discord — `{4}{B}{B}` 6/6 Demon.
//! Flying.
//! As this creature enters, choose two players.
//! Whenever damage is dealt to one of the chosen players, the other chosen
//! player also loses that much life.
//!
//! Flying is a base keyword. The ETB "choose two players" and the linked
//! damage-mirror trigger are GAP'd:
//!  - There is no primitive to choose and persist a pair of players.
//!  - There is no trigger condition for "damage dealt to one of two chosen
//!    players" feeding the other's life loss.

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
    let name = reg.interner_mut().intern("Sower of Discord");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: choose_two_players_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP (trigger): "Whenever damage is dealt to one of the chosen
        // players, the other also loses that much life" has no matching
        // condition and no persisted chosen-pair state — omitted.
    )
}

fn choose_two_players_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose two players" cannot be selected/persisted, and the
    // dependent damage-mirror trigger is unexpressible.
    Vec::new()
}
