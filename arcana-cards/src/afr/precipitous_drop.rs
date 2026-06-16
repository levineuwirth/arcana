//! Precipitous Drop — `{2}{B}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, venture into the dungeon.
//!  Enchanted creature gets -2/-2. It gets -5/-5 instead as long as you've
//!  completed a dungeon."
//!
//! The base debuff is an ETB-installed `attached_pt(-2, -2)`. The ETB
//! "venture into the dungeon" rider and the conditional "-5/-5 instead as
//! long as you've completed a dungeon" upgrade have no expressible
//! primitives here — GAP'd; the static -2/-2 is faithful.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Precipitous Drop");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
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
    // GAP: "venture into the dungeon" ETB rider — no venture primitive here.
    // GAP: conditional "-5/-5 instead as long as you've completed a dungeon" —
    //      no as-long-as upgrade primitive; the static -2/-2 is installed.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            -2,
            -2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
