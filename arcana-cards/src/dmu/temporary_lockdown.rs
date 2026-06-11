//! Temporary Lockdown — `{1}{W}{W}` enchantment.
//! "When this enchantment enters, exile each nonland permanent with
//! mana value 2 or less until this enchantment leaves the battlefield."
//!
//! ETB sweep over `script::ids_matching` + `Effect::ForEach`. GAP: the
//! "until this enchantment leaves the battlefield" return half is not
//! expressible; the exile is permanent here.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Temporary Lockdown");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_cheap_nonlands,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…exile each nonland permanent with mana value 2 or less until this
/// enchantment leaves the battlefield."
fn etb_exile_cheap_nonlands(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::permanent()
        .without_types(TypeLine::LAND.into())
        .with_max_cmc(2);
    let ids = script::ids_matching(state, &filter, trig.controller);
    // GAP: "until this enchantment leaves the battlefield" — the
    // return-on-leave half of the O-ring is not expressible; the exile
    // is permanent here.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ExilePermanent {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
