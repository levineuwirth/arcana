//! Cruel Somnophage // Can't Wake Up — `{1}{B}` // `{1}{U}` black Adventure creature.
//! Creature: *//* Nightmare. "P/T = number of creature cards in all graveyards."
//! Adventure (Can't Wake Up — Sorcery): Target player mills four cards.
//! The */* P/T is wired as a Layer-7a self-CDA (`self_pt_cda`, creature cards in
//!   all graveyards) installed on `SelfEntersBattlefield` with
//!   `Duration::WhileSourceOnBattlefield`; bones are `PtValue::Star`.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::{Zone, ZoneKind};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cruel Somnophage");
    let adv_name = reg.interner_mut().intern("Can't Wake Up");
    let nightmare_sub = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        // */* — defined by the CDA (creature cards in all graveyards) installed below.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::SORCERY.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "Target player mills four cards.".into(), target_requirements: vec![TargetRequirement::target_player()], modal: None, effect: cant_wake_up_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_adventure(adventure)
            // CDA: P/T each equal to the number of creature cards in all graveyards.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Layer 7a self-CDA: P/T each equal to the number of creature cards in all graveyards.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            creature_cards_in_all_graveyards,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// P/T = the number of creature cards in all players' graveyards.
fn creature_cards_in_all_graveyards(s: &GameState, _source: ObjectId) -> (i32, i32) {
    let n = s
        .objects
        .objects_in_zone_kind(ZoneKind::Graveyard)
        .filter(|o| o.characteristics.types.is_creature())
        .count() as i32;
    (n, n)
}

fn cant_wake_up_resolve(_state: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::Mill { player: *p, count: 4 }]
}
