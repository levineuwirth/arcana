//! Vulshok War Boar — `{2}{R}{R}` 5/5 red Creature — Boar Beast.
//! "When this creature enters, sacrifice it unless you sacrifice an artifact."
//! GAP: conditional cost ("unless you sacrifice an artifact") not expressible
//! in the Effect catalog; sacrificing self unconditionally as approximation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vulshok War Boar");
    let boar = reg.interner_mut().intern("Boar");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(boar);
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: "unless you sacrifice an artifact" conditional cost not
                // expressible; sacrificing self unconditionally as approximation
                intervening_if: None,
                effect: etb_sacrifice_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_sacrifice_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should sacrifice an artifact as an alternative cost; approximated
    // as unconditional self-sacrifice
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::permanent(),
        count: 1,
    }]
}
