//! Fear of Death — `{1}{U}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, mill two cards. Enchanted
//!  creature gets -X/-0, where X is the number of cards in your graveyard."
//!
//! The ETB mills two cards (`Effect::Mill`) and installs a dynamic P/T
//! modifier whose compute fn reads the SOURCE (the Aura) to count cards in
//! its controller's graveyard, applying -X/-0. Zone-size dynamic P/T is
//! fully expressible via `attached_pt_dynamic` + `script::graveyard_size`.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Fear of Death");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Mill {
            player: trig.controller,
            count: 2,
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt_dynamic(
                trig.source,
                minus_graveyard_size,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

fn minus_graveyard_size(state: &GameState, source: ObjectId) -> (i32, i32) {
    let Some(controller) = state.object_or_lki(source).map(|o| o.controller) else {
        return (0, 0);
    };
    let x = script::graveyard_size(state, controller) as i32;
    (-x, 0)
}
