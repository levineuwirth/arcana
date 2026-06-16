//! Lush Growth — `{G}` enchantment — Aura.
//! "Enchant land. Enchanted land is a Mountain, Forest, and Plains."
//!
//! Full (with caveat): the three land subtypes are added via attached_subtypes.
//! NOTE: the printed card REPLACES the land's types; attached_subtypes is
//! additive, the nearest available builder (cf. Spreading Seas).

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
    let name = reg.interner_mut().intern("Lush Growth");
    let aura = reg.interner_mut().intern("Aura");
    // intern granted land subtypes for effect-time lookup.
    let _mountain = reg.interner_mut().intern("Mountain");
    let _forest = reg.interner_mut().intern("Forest");
    let _plains = reg.interner_mut().intern("Plains");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
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

fn etb_install(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let mut grant = SubtypeSet::default();
    if let Some(m) = reg.interner().lookup("Mountain") {
        grant.0.insert(m);
    }
    if let Some(f) = reg.interner().lookup("Forest") {
        grant.0.insert(f);
    }
    if let Some(p) = reg.interner().lookup("Plains") {
        grant.0.insert(p);
    }
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_subtypes(
            trig.source,
            grant,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
