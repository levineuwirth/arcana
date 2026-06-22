//! Nightshade Assassin — `{2}{B}{B}` 2/1 black Human Assassin with First strike.
//!
//! * First strike.
//! * When this creature enters, you may reveal X black cards in your hand. If
//!   you do, target creature gets -X/-X until end of turn.
//! * Madness {1}{B}.  (Madness is not an expressible keyword; GAP'd.)
//!
//! First strike is expressed faithfully. The ETB trigger is GAP'd: X is the
//! player-chosen number of revealed black cards, and there is no primitive to
//! count a variable reveal and feed it into a -X/-X pump. Madness has no
//! `KeywordAbility` variant.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nightshade Assassin");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: "Madness {1}{B}" — Madness has no usable KeywordAbility variant.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_reveal_black,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn etb_reveal_black(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal X black cards in your hand. If you do, target creature gets
    // -X/-X" — X is the player-chosen count of revealed black cards; no
    // primitive counts a variable reveal and applies a -X/-X pump.
    Vec::new()
}
