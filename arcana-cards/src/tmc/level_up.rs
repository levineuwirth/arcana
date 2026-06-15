//! Level Up — `{1}{G}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, put a +1/+1 counter on enchanted
//! creature. Enchanted creature has \"Whenever this creature attacks, double
//! the number of +1/+1 counters on it. Then if it has power 10 or greater,
//! draw a card.\""
//!
//! The ETB +1/+1 counter on the host is expressible via AddCounters on the
//! attached object. The granted attacks-trigger ("double counters, then draw
//! if power 10+") has no demonstrated builder for granting a triggered ability
//! that doubles counters — that's a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Level Up");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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

fn etb_install(state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // Put a +1/+1 counter on the enchanted creature (host).
    // GAP: granted attacks-trigger ("double the +1/+1 counters on it, then if
    // power 10+ draw a card") has no attached/grant builder for doubling.
    let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: host,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
