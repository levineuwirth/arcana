//! Kagemaro's Clutch — `{3}{B}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets -X/-X, where X is the number
//!  of cards in your hand."
//!
//! Dynamic debuff Aura. ETB installs `attached_pt_dynamic` whose compute
//! fn reads the SOURCE (the Aura) to find its controller and counts the
//! cards in that player's hand (a zone size, fully expressible), returning
//! (-X, -X).

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
    let name = reg.interner_mut().intern("Kagemaro's Clutch");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
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
            minus_cards_in_hand,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn minus_cards_in_hand(state: &GameState, source: ObjectId) -> (i32, i32) {
    let Some(controller) = state.object_or_lki(source).map(|o| o.controller) else {
        return (0, 0);
    };
    let n = script::hand_size(state, controller) as i32;
    (-n, -n)
}
