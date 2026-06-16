//! Reprobation — `{1}{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature loses all abilities and is a
//!  Coward creature with base power and toughness 0/1."
//!
//! Base P/T set to 0/1 via `attached_set_pt`; the Coward creature type
//! added via `attached_subtypes`. "Loses all abilities" has no builder in
//! the demonstrated surface — partially expressed.

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
    let name = reg.interner_mut().intern("Reprobation");
    let aura = reg.interner_mut().intern("Aura");
    let _coward = reg.interner_mut().intern("Coward");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses all abilities" has no attached_* builder; base 0/1 and the
    // Coward type are installed.
    let coward = reg.interner().lookup("Coward").expect("Coward interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(coward);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_set_pt(
                trig.source,
                0,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                subtypes,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
