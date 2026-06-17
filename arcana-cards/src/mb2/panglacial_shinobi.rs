//! Panglacial Shinobi — `{2}{U}` 1/3 Wurm Ninja (U).
//! Library ninjutsu {1}{U} (cast-from-library combat trick) is not in the keyword
//! surface — GAP'd. Combat-damage-to-a-player draw is wired via DamageDealt, with the
//! source restricted by name as the closest expressible "this creature" filter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Panglacial Shinobi");
    let wurm = reg.interner_mut().intern("Wurm");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);
    subtypes.0.insert(ninja);

    // GAP: Library ninjutsu {1}{U} — cast-from-library combat keyword not in the
    // KeywordAbility / ActivationCost surface; omitted.

    let self_name = reg.interner().lookup("Panglacial Shinobi");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter { name: self_name, ..ObjectFilter::default() },
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: draw_a_card,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_a_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
