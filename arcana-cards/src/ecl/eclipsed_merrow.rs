//! Eclipsed Merrow — `{W/U}{W/U}{W/U}` 2/3 white/blue Merfolk Scout.
//! "When this creature enters, look at the top four cards of your library. You may reveal a Merfolk,
//! Plains, or Island card from among them and put it into your hand. Put the rest on the bottom
//! of your library in a random order."
//! GAP: look-at-top-N and conditional selective reveal not expressible; emitting Scry 4 as proxy.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eclipsed Merrow");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W/U}{W/U}{W/U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: look_top_four,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn look_top_four(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: look at top 4, conditionally reveal Merfolk/Plains/Island to hand, rest to bottom
    // not expressible; using Scry 4 as proxy
    vec![Effect::Scry { player: trig.controller, count: 4 }]
}
