//! Prince of Thralls — `{4}{U}{B}{B}{R}` 7/7 blue-black-red Demon.
//! "Whenever a permanent an opponent controls is put into a graveyard, put
//! that card onto the battlefield under your control unless that opponent
//! pays 3 life."
//!
//! GAP: "unless opponent pays 3 life" — optional cost for opponent is not
//! expressible. Emitting ChangeControl (steal the dying permanent) without
//! the opt-out clause.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prince of Thralls");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::permanent()
                    .controlled_by(ControllerConstraint::Opponent),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: on_opponent_permanent_dies,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_opponent_permanent_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.dying_object() else {
        return Vec::new();
    };
    // GAP: "unless that opponent pays 3 life" opt-out not expressible.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: id }]
}
