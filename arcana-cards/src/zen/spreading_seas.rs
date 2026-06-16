//! Spreading Seas — `{1}{U}` enchantment — Aura.
//! "Enchant land.
//!  When this Aura enters, draw a card.
//!  Enchanted land is an Island."
//!
//! On ETB the controller draws a card. The land-type change is an
//! attached_subtypes(Island) install resolved from the interner at
//! effect time. NOTE: the printed card REPLACES the land's types with
//! Island; attached_subtypes is additive, the nearest available builder.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spreading Seas");
    let aura = reg.interner_mut().intern("Aura");
    // intern Island now so the symbol exists for the effect-time lookup.
    let _island = reg.interner_mut().intern("Island");
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
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            ))
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut grant = SubtypeSet::default();
    if let Some(island) = reg.interner().lookup("Island") {
        grant.0.insert(island);
    }
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                grant,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
