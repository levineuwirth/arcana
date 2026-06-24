//! Swamp Mosquito — `{1}{B}` 0/1 Insect.
//! Flying.
//! "Whenever this creature attacks and isn't blocked, defending player
//! gets a poison counter."

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
    let name = reg.interner_mut().intern("Swamp Mosquito");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacksUnblocked,
            intervening_if: None,
            effect: defending_player_gets_poison,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn defending_player_gets_poison(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "defending player gets a poison counter." SelfAttacksUnblocked fires on
    // CreatureNotBlocked (no defender on the event), so read the attacker's
    // defending_player from combat state, falling back to trig.defending_player().
    let player = state
        .combat
        .as_ref()
        .and_then(|c| c.attacker(trig.source))
        .map(|a| a.defending_player)
        .or_else(|| trig.defending_player());
    let Some(player) = player else { return Vec::new(); };
    vec![Effect::GivePlayerCounters { player, kind: CounterKind::Poison, count: 1 }]
}
