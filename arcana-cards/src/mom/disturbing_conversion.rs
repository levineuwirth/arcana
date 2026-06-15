//! Disturbing Conversion — `{1}{U}` enchantment — Aura.
//! "Flash. Enchant creature. When this Aura enters, each player mills two
//!  cards. Enchanted creature gets -X/-0, where X is the number of cards in
//!  its controller's graveyard."
//!
//! Flash is a printed keyword. The ETB mills two for every player. The static
//! debuff is an `attached_pt_dynamic`: the compute fn finds the host via
//! `source.attached_to`, then subtracts the host controller's graveyard size
//! from power.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Disturbing Conversion");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill_and_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_mill_and_install(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 2 })
        .collect();
    effects.push(Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt_dynamic(
            trig.source,
            graveyard_debuff,
            Duration::WhileSourceOnBattlefield,
        ),
    });
    effects
}

fn graveyard_debuff(state: &GameState, source: ObjectId) -> (i32, i32) {
    let Some(host) = state.object_or_lki(source).and_then(|o| o.attached_to) else {
        return (0, 0);
    };
    let Some(controller) = state.object_or_lki(host).map(|o| o.controller) else {
        return (0, 0);
    };
    let x = script::graveyard_size(state, controller) as i32;
    (-x, 0)
}
