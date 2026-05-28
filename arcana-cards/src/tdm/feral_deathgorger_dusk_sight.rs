//! Feral Deathgorger // Dusk Sight — `{5}{B}` // `{1}{B}` black Adventure creature.
//! Creature: 3/5 Dragon. Flying, deathtouch. When this creature enters, exile up to two target cards from a single graveyard.
//! Adventure (Dusk Sight — Sorcery): Put a +1/+1 counter on up to one target creature. Draw a card.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement, ObjectFilter};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feral Deathgorger");
    let adv_name = reg.interner_mut().intern("Dusk Sight");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);
    let main_chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")), colors: ColorSet::black(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(3)), toughness: Some(PtValue::Fixed(5)), keywords: vec![KeywordAbility::Flying, KeywordAbility::Deathtouch], ..Default::default() };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")), colors: ColorSet::black(), types: TypeLine::SORCERY.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "Put a +1/+1 counter on up to one target creature. Draw a card.".into(), target_requirements: vec![TargetRequirement { filter: TargetFilter::Creature, count: TargetCount::UpTo(1), controller: None }], modal: None, effect: dusk_sight_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_triggered_ability(TriggeredAbilityDef { id: 1, trigger_condition: TriggerCondition::SelfEntersBattlefield, intervening_if: None, effect: etb_exile, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement { filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::new() }, count: TargetCount::UpTo(2), controller: None }] })
            .with_adventure(adventure),
    )
}

fn etb_exile(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    trig.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t { Some(Effect::ExileFromGraveyard { target: *id }) } else { None }
    }).collect()
}

fn dusk_sight_resolve(_state: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    let mut effects: Vec<Effect> = entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t { Some(Effect::AddCounters { target: *id, kind: CounterKind::PlusOnePlusOne, count: 1 }) } else { None }
    }).collect();
    effects.push(Effect::DrawCards { player: entry.controller, count: 1 });
    effects
}
