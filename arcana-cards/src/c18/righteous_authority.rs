//! Righteous Authority — `{3}{W}{U}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +1/+1 for each card in its
//!  controller's hand. At the beginning of the draw step of enchanted
//!  creature's controller, that player draws an additional card."
//!
//! The buff is an ETB-installed `attached_pt_dynamic` reaching the host
//! (`source.attached_to`), then counting that controller's hand. The
//! additional-draw clause keys on the HOST controller's draw step, but no
//! StepBegins constraint can name "the enchanted creature's controller"
//! (only You/Opponent/Any) — GAP.

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
    let name = reg.interner_mut().intern("Righteous Authority");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
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
    // GAP: "at the beginning of the draw step of enchanted creature's
    // controller, that player draws an additional card" — StepBegins can
    // only key on You/Opponent/Any, not the host's controller.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt_dynamic(
            trig.source,
            plus_cards_in_host_controller_hand,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn plus_cards_in_host_controller_hand(
    state: &GameState,
    source: ObjectId,
) -> (i32, i32) {
    let Some(host) = state.object_or_lki(source).and_then(|o| o.attached_to) else {
        return (0, 0);
    };
    let Some(controller) = state.object_or_lki(host).map(|o| o.controller) else {
        return (0, 0);
    };
    let n = script::hand_size(state, controller) as i32;
    (n, n)
}
