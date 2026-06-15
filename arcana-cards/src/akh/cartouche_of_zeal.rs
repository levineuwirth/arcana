//! Cartouche of Zeal — `{R}` enchantment — Aura Cartouche.
//! "Enchant creature you control. When this Aura enters, target creature can't
//! block this turn. Enchanted creature gets +1/+1 and has haste."
//!
//! Buff Aura: the ETB installs `attached_pt` (+1/+1) and `attached_keyword`
//! (Haste). GAP: the separate ETB clause "target creature can't block this
//! turn" is a one-shot targeted effect on a non-host creature with no
//! demonstrated Effect variant.

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
    let name = reg.interner_mut().intern("Cartouche of Zeal");
    let aura = reg.interner_mut().intern("Aura");
    let cartouche = reg.interner_mut().intern("Cartouche");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    subtypes.0.insert(cartouche);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    // NOTE: controller wording ("enchant creature you control") approximated by
    // caster's choice.
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

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: ETB "target creature can't block this turn" — targeted one-shot on a
    // non-host creature, no demonstrated Effect variant.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(
                trig.source,
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Haste,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
