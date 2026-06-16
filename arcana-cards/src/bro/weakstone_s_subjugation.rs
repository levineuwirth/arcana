//! Weakstone's Subjugation — `{U}` enchantment — Aura.
//! "Enchant artifact or creature. When this Aura enters, you may pay {3}.
//!  If you do, tap enchanted permanent. Enchanted permanent doesn't untap
//!  during its controller's untap step."
//!
//! The persistent effect — enchanted permanent doesn't untap — is an
//! ETB-installed `attached_dont_untap` continuous effect that follows
//! `source.attached_to`. The one-time "you may pay {3}, then tap it" rider
//! is an optional payment whose tap target is the host; that optional cost
//! shape is not expressible with the shown ETB-install API, so it's a GAP.
//!
//! NOTE: artifact-or-creature widened to any permanent.
//! GAP: "you may pay {3}. If you do, tap enchanted permanent." optional rider.

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
    let name = reg.interner_mut().intern("Weakstone's Subjugation");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(ObjectFilter::permanent()))
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
    // GAP: optional "pay {3}, then tap enchanted permanent" rider not expressible.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_dont_untap(
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
