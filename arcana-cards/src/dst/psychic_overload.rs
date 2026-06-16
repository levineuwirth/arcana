//! Psychic Overload — `{3}{U}` enchantment — Aura.
//! "Enchant permanent. When this Aura enters, tap enchanted permanent.
//!  Enchanted permanent doesn't untap during its controller's untap step.
//!  Enchanted permanent has 'Discard two artifact cards: Untap this
//!  permanent.'"
//!
//! ETB taps the host (`source.attached_to`) and installs
//! `attached_dont_untap`. The granted "Discard two artifact cards: Untap"
//! ability has a non-mana discard cost that ActivationCost can't express —
//! GAP.

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
    let name = reg.interner_mut().intern("Psychic Overload");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
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

fn etb_install(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granted "Discard two artifact cards: Untap this permanent" has a
    // non-mana discard activation cost that ActivationCost can't express.
    let mut effects = Vec::new();
    if let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) {
        effects.push(Effect::Tap { target: host });
    }
    effects.push(Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_dont_untap(
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    });
    effects
}
