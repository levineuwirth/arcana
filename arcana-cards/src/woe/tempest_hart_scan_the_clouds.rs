//! Tempest Hart // Scan the Clouds — `{3}{G}` // `{1}{U}` green Adventure creature.
//! Creature: 3/4 Elemental Elk. Trample. "Whenever you cast a spell with mana value 5 or greater, put a +1/+1 counter on this creature."
//! Adventure (Scan the Clouds — Instant): Draw two cards, then discard two cards.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tempest Hart");
    let adv_name = reg.interner_mut().intern("Scan the Clouds");
    let elemental_sub = reg.interner_mut().intern("Elemental");
    let elk_sub = reg.interner_mut().intern("Elk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental_sub);
    subtypes.0.insert(elk_sub);
    let main_chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(3)), toughness: Some(PtValue::Fixed(4)), keywords: vec![KeywordAbility::Trample], ..Default::default() };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::INSTANT.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "Draw two cards, then discard two cards.".into(), target_requirements: vec![], modal: None, effect: scan_the_clouds_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast { filter: Some(ObjectFilter::new().with_min_cmc(5)), caster: ControllerConstraint::You },
                intervening_if: None, effect: on_big_spell, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new(),
            })
            .with_adventure(adventure),
    )
}

fn on_big_spell(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::PlusOnePlusOne, count: 1 }]
}

fn scan_the_clouds_resolve(_state: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        Effect::Discard { player: entry.controller, count: 2, choice: DiscardChoice::ControllerChooses },
    ]
}
