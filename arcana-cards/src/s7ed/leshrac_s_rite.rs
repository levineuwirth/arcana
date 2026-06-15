//! Leshrac's Rite — `{B}` enchantment — Aura.
//! "Enchant creature. Enchanted creature has swampwalk."
//!
//! Grants the host swampwalk via an `attached_keyword` install of
//! `KeywordAbility::Landwalk(Swamp)`. "Swamp" is interned in `register`
//! and re-looked-up in the ETB effect fn (which carries the registry).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Leshrac's Rite");
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
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_swampwalk,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_swampwalk(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(swamp) = reg.interner().lookup("Swamp") else {
        return Vec::new();
    };
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_keyword(
            trig.source,
            KeywordAbility::Landwalk(swamp),
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
