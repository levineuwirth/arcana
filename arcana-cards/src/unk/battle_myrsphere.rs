//! Battle Myrsphere — `{7}` 4/7 Artifact Creature — Phyrexian Myr.
//!
//! When Battle Myrsphere enters the battlefield, reveal cards from the
//! top of your library until you reveal a battle card. Put it onto the
//! battlefield, and the rest on the bottom of your library in a random
//! order.
//! Whenever Battle Myrsphere attacks, you may tap X untapped battles
//! you control. If you do, Battle Myrsphere gets +X/+0 until end of
//! turn and deals X damage to the player or permanent.
//!
//! The ETB is a `RevealUntil` (find a battle card → onto the
//! battlefield, rest to the bottom). The attack ability is GAP'd: its
//! X is the player-chosen number of battles tapped as a resolution-time
//! cost, and there is no demonstrated primitive for a variable
//! player-chosen tap that simultaneously feeds a pump and a divided
//! damage amount.

use arcana_core::effects::{Effect, RevealDest, DigRest};
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
    let name = reg.interner_mut().intern("Battle Myrsphere");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let myr = reg.interner_mut().intern("Myr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(myr);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_reveal_battle,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_tap_battles,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: reveal until a battle card, put it onto the battlefield, rest
/// to the bottom of the library in a random order.
fn etb_reveal_battle(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::BATTLE.into()),
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: None,
    }]
}

/// Attack ability. GAP: "you may tap X untapped battles you control. If
/// you do, ~ gets +X/+0 and deals X damage to the player or permanent."
/// X is a player-chosen variable tap count paid at resolution that
/// simultaneously sizes a pump and a damage payload — no demonstrated
/// primitive expresses a variable player-chosen tap feeding both.
fn attack_tap_battles(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}
