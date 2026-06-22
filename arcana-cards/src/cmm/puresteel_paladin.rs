//! Puresteel Paladin — `{W}{W}` 2/2 Creature — Human Knight.
//! Whenever an Equipment you control enters, you may draw a card.
//! Metalcraft — Equipment you control have equip {0} as long as you control
//! three or more artifacts.
//!
//! The Equipment-enters trigger is wired (ZoneChange to battlefield, filtered
//! to Equipment you control); the "may draw" is resolved as a draw (the
//! optional is a resolution-time choice). The Metalcraft static (granting
//! equip {0} to other Equipment while you control 3+ artifacts) is a
//! conditional ability-granting static with no demonstrated primitive and is
//! GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Puresteel Paladin");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let equipment_filter = script::subtype_filter(reg, "Equipment").controlled_by(ControllerConstraint::You);

    // GAP: "Metalcraft — Equipment you control have equip {0} as long as you
    // control three or more artifacts." — a conditional ability-granting
    // continuous static, no demonstrated primitive.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: equipment_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: may_draw_a_card,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn may_draw_a_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
