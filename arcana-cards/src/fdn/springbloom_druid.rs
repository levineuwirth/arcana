//! Springbloom Druid — `{2}{G}` 1/1 green Elf Druid.
//! "When this creature enters, you may sacrifice a land. If you do, search your library
//! for up to two basic land cards, put them onto the battlefield tapped, then shuffle."
//! Best-effort: Sacrifice(land) + TutorToBattlefield(basic land, tapped) twice.
//! GAP: "up to two" is approximated as exactly two TutorToBattlefield effects.

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
    let name = reg.interner_mut().intern("Springbloom Druid");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_sacrifice_land_fetch,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_sacrifice_land_fetch(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Sacrifice a land, then search for up to two basic lands (tapped).
    // GAP: "you may sacrifice a land" optional cost not expressible;
    // GAP: "up to two" approximated as two TutorToBattlefield effects.
    let land_filter = ObjectFilter::new().with_types(TypeLine::LAND.into());
    vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: land_filter.clone(),
            count: 1,
        },
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: land_filter.clone(),
            tapped: true,
        },
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: land_filter,
            tapped: true,
        },
    ]
}
