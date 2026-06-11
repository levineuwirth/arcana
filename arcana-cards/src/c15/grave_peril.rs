//! Grave Peril — `{1}{B}` enchantment.
//! "When a nonblack creature enters, sacrifice this enchantment. If you
//! do, destroy that creature."
//!
//! A battlefield-bound ZoneChange trigger on nonblack creatures. The
//! sacrifice-self half is approximated (no sacrifice-self effect
//! variant); the destroy reads the entering creature off the trigger.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grave Peril");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().without_colors(ColorSet::black()),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: sac_and_destroy,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "…sacrifice this enchantment. If you do, destroy that creature."
fn sac_and_destroy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(entered) = trig.entering_object() else {
        return Vec::new();
    };
    // GAP: "sacrifice this enchantment. If you do, ..." — there is no
    // sacrifice-SELF effect variant (Effect::Sacrifice is a filtered
    // player choice), so the sacrifice is approximated as destroying the
    // source, and the destroy of the entering creature is not conditioned
    // on the sacrifice actually happening.
    vec![
        Effect::DestroyPermanent { target: trig.source },
        Effect::DestroyPermanent { target: entered },
    ]
}
