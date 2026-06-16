//! Death's Approach — `{B}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets -X/-X, where X is the number
//!  of creature cards in its controller's graveyard."
//!
//! Dynamic debuff: an ETB-installed `attached_pt_dynamic` whose compute fn
//! reaches the host (`source.attached_to`), takes the HOST controller's
//! graveyard, counts creature cards there (a type-only filter, fully
//! expressible), and returns (-X, -X).

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
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Death's Approach");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
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
            minus_creatures_in_host_controller_graveyard,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn minus_creatures_in_host_controller_graveyard(
    state: &GameState,
    source: ObjectId,
) -> (i32, i32) {
    let Some(host) = state.object_or_lki(source).and_then(|o| o.attached_to) else {
        return (0, 0);
    };
    let Some(controller) = state.object_or_lki(host).map(|o| o.controller) else {
        return (0, 0);
    };
    let n = script::graveyard_matching(
        state,
        &ObjectFilter::creature(),
        controller,
        controller,
    ) as i32;
    (-n, -n)
}
