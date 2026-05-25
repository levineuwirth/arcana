//! Dread Slaver — `{3}{B}{B}` 3/5 black Zombie Horror. "Whenever a creature dealt
//! damage by this creature this turn dies, return it to the battlefield under your
//! control. That creature is a black Zombie in addition to its other colors and types."
//! GAP: "creature dealt damage by this creature this turn" tracking not in trigger catalog;
//! using ZoneChange(BF->GY) as closest approximation. Type/color modification not in catalog.

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
    let name = reg.interner_mut().intern("Dread Slaver");
    let zombie = reg.interner_mut().intern("Zombie");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "creature dealt damage by ~ this turn" tracking not in trigger catalog
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: on_creature_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_creature_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.dying_object() else { return Vec::new(); };
    // GAP: type/color modification (becomes black Zombie) not in engine effect catalog
    vec![Effect::ReturnFromGraveyardToBattlefield { target: id }]
}
