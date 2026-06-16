//! Evil Presence — `{B}` enchantment — Aura.
//! "Enchant land. Enchanted land is a Swamp."
//!
//! Enchant land via `with_enchant(TargetFilter::Permanent(land))`. The
//! "is a Swamp" grant is an ETB-installed `attached_subtypes` continuous
//! effect adding the Swamp subtype to the host. (Oracle "is a Swamp"
//! replaces the land's existing subtypes; the additive `attached_subtypes`
//! builder approximates by adding Swamp.)

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
    let name = reg.interner_mut().intern("Evil Presence");
    let aura = reg.interner_mut().intern("Aura");
    let _swamp = reg.interner_mut().intern("Swamp");
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
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            ))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_swamp,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_swamp(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let swamp = reg.interner().lookup("Swamp").unwrap_or_default();
    let mut s = SubtypeSet::default();
    s.0.insert(swamp);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_subtypes(
            trig.source,
            s,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
