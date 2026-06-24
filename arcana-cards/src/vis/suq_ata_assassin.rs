//! Suq'Ata Assassin — `{1}{B}{B}` 1/1 Human Assassin with Fear.
//! "Whenever this creature attacks and isn't blocked, defending player gets a
//! poison counter."
//!
//! Fear is a base keyword. The unblocked-attack trigger fires; the defending
//! player gets a poison counter via Effect::GivePlayerCounters { kind: Poison }.
//! SelfAttacksUnblocked fires on CreatureNotBlocked (no defending player on the
//! event), so the defender is read from combat state via the source attacker,
//! falling back to trig.defending_player().

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Suq'Ata Assassin");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Fear],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacksUnblocked,
                intervening_if: None,
                effect: poison_defending_player,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn poison_defending_player(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "defending player gets a poison counter." The defender is the attacker's
    // defending_player in combat state (the CreatureNotBlocked event has none).
    let player = state
        .combat
        .as_ref()
        .and_then(|c| c.attacker(trig.source))
        .map(|a| a.defending_player)
        .or_else(|| trig.defending_player());
    let Some(player) = player else { return Vec::new(); };
    vec![Effect::GivePlayerCounters { player, kind: CounterKind::Poison, count: 1 }]
}
