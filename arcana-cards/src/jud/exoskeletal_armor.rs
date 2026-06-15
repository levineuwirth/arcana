//! Exoskeletal Armor — `{1}{G}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +X/+X, where X is the number of
//!  creature cards in all graveyards."
//!
//! Dynamic buff: an ETB-installed `attached_pt_dynamic` whose compute fn
//! sums creature cards across every player's graveyard (a type-only filter,
//! fully expressible) and returns (X, X).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Exoskeletal Armor");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt_dynamic(
            trig.source,
            creatures_in_all_graveyards,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn creatures_in_all_graveyards(state: &GameState, source: ObjectId) -> (i32, i32) {
    let you = state.object_or_lki(source).map(|o| o.controller).unwrap_or(0);
    let mut total: u32 = 0;
    for p in 0..state.num_players() {
        total += script::graveyard_matching(
            state,
            &ObjectFilter::creature(),
            p as PlayerId,
            you,
        );
    }
    (total as i32, total as i32)
}
