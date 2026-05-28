//! Murderous Rider // Swift End — `{1}{B}{B}` // `{1}{B}{B}` black Adventure creature.
//! Creature: 2/3 Zombie Knight. Lifelink. When this creature dies, put it on the bottom of its owner's library.
//! Adventure (Swift End — Instant): Destroy target creature or planeswalker. You lose 2 life.
//! GAP: "destroy target creature or planeswalker" — DestroyPermanent works for creatures; planeswalker type not separately modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Murderous Rider");
    let adv_name = reg.interner_mut().intern("Swift End");
    let zombie_sub = reg.interner_mut().intern("Zombie");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie_sub);
    subtypes.0.insert(knight_sub);
    let main_chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")), colors: ColorSet::black(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(3)), keywords: vec![KeywordAbility::Lifelink], ..Default::default() };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")), colors: ColorSet::black(), types: TypeLine::INSTANT.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "Destroy target creature or planeswalker. You lose 2 life.".into(), target_requirements: vec![TargetRequirement::target_creature()], modal: None, effect: swift_end_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_triggered_ability(TriggeredAbilityDef { id: 1, trigger_condition: TriggerCondition::SelfDies, intervening_if: None, effect: dies_put_on_bottom, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_adventure(adventure),
    )
}

fn dies_put_on_bottom(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::PutOnBottomOfLibrary { target: trig.source }]
}

fn swift_end_resolve(_state: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::LoseLife { player: entry.controller, amount: 2 },
    ]
}
