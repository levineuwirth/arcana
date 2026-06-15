//! Encase in Ice — `{1}{U}` enchantment — Aura.
//! "Flash. Enchant red or green creature. When this Aura enters, tap
//!  enchanted creature. Enchanted creature doesn't untap during its
//!  controller's untap step."
//!
//! Flash is a recognized keyword. The ETB trigger taps the host (reached
//! via `source.attached_to`, the Runner's Bane exemplar). The "doesn't
//! untap during its controller's untap step" restriction has no
//! attached_* builder, so that clause is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Encase in Ice");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(
        // NOTE: "enchant red or green creature" approximated to any creature
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
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "doesn't untap during its controller's untap step" lock has no
    // attached_* builder; only the ETB tap is expressed.
    let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    vec![Effect::Tap { target: host }]
}
