//! Meanders Guide — `{2}{W}` 3/2 white creature (Merfolk Scout).
//! "Whenever this creature attacks, you may tap another untapped Merfolk you
//! control. When you do, return target creature card with mana value 3 or
//! less from your graveyard to the battlefield."
//!
//! GAP: "you may tap another untapped Merfolk you control; when you do" —
//! optional tap-a-creature condition is not expressible. Emitting the
//! Reanimate effect with CMC filter as best effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Meanders Guide");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "tap another untapped Merfolk you control; if you do" optional
    // condition not expressible; emitting Reanimate unconditionally.
    vec![Effect::Reanimate {
        player: trig.controller,
        filter: arcana_core::targets::ObjectFilter::creature().with_max_cmc(3),
        from_zone: Zone::Graveyard(trig.controller),
    }]
}
