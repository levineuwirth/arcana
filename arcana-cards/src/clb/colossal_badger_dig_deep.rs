//! Colossal Badger // Dig Deep — `{5}{G}` // `{1}{G}` green Adventure creature.
//! Creature: 6/5 Badger. When this creature enters, you gain 3 life.
//! Adventure (Dig Deep — Sorcery): Choose target creature. Mill four cards, then put a +1/+1 counter
//!   on that creature for each creature card milled this way.
//! GAP: Dig Deep "for each creature card milled" — count creature cards among milled cards not in script API.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Colossal Badger");
    let adv_name = reg.interner_mut().intern("Dig Deep");
    let badger_sub = reg.interner_mut().intern("Badger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(badger_sub);
    let main_chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(6)), toughness: Some(PtValue::Fixed(5)), ..Default::default() };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::SORCERY.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "Choose target creature. Mill four cards, then put a +1/+1 counter on it for each creature card milled.".into(), target_requirements: vec![TargetRequirement::target_creature()], modal: None, effect: dig_deep_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_triggered_ability(TriggeredAbilityDef { id: 1, trigger_condition: TriggerCondition::SelfEntersBattlefield, intervening_if: None, effect: etb_gain_life, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_adventure(adventure),
    )
}

fn etb_gain_life(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife { player: trig.controller, amount: 3 }]
}

fn dig_deep_resolve(_state: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "+1/+1 for each creature card milled" — count creatures among milled cards not in script API
    vec![
        Effect::Mill { player: entry.controller, count: 4 },
        // Emitting a fixed +1/+1 counter as best-effort; actual count is dynamic
        Effect::AddCounters { target: *id, kind: CounterKind::PlusOnePlusOne, count: 1 },
    ]
}
