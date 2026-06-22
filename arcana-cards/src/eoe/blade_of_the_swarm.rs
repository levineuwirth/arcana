//! Blade of the Swarm — `{3}{B}` 3/1 Insect Assassin.
//!
//! When this creature enters, choose one —
//! • Put two +1/+1 counters on this creature.
//! • Put target exiled card with warp on the bottom of its owner's library.
//!
//! `TriggeredAbilityDef` has no modal field, so the ETB "choose one"
//! can't be posted as a player choice. The second mode targets an
//! exiled card filtered by the Warp marker, which is also not
//! expressible (no exile-zone card target / warp filter). We wire the
//! unconditional first mode (two +1/+1 counters on this creature) and
//! GAP the modal choice + second mode.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blade of the Swarm");
    let insect = reg.interner_mut().intern("Insect");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            // GAP: modal "choose one" not expressible on a triggered ability;
            // GAP: mode 2 "put target exiled card with warp on the bottom" —
            //      no exile-zone card target / warp filter. We resolve mode 1.
            effect: etb_two_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_two_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}
