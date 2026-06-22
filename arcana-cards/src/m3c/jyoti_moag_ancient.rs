//! Jyoti, Moag Ancient — `{2}{G}{U}` 2/4 Legendary Elemental (G/U).
//!
//! Oracle:
//! * When Jyoti enters, create a 1/1 green Forest Dryad land creature token
//!   for each time you've cast your commander from the command zone this game.
//!   (GAP — "times you've cast your commander from the command zone this game"
//!   has no script accessor; the count is uncomputable with the demonstrated
//!   API. The token shape itself is fine, but the dynamic count is not.)
//! * At the beginning of each combat, land creatures you control get +X/+X
//!   until end of turn, where X is Jyoti's power.  (Wired — phase-begins
//!   trigger pumps every land creature you control by Jyoti's power.)

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jyoti, Moag Ancient");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // ETB: create N Forest Dryad land-creature tokens — GAP (count).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dryads_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // At the beginning of each combat, land creatures you control get
            // +X/+X UEOT where X is Jyoti's power.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: combat_pump_land_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_dryads_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "for each time you've cast your commander from the command zone this
    // game" — no script accessor for commander-cast count; cannot compute N.
    Vec::new()
}

fn combat_pump_land_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::power_of(state, trig.source).max(0);
    let filter = ObjectFilter::permanent()
        .with_types(TypeLine(TypeLine::LAND | TypeLine::CREATURE))
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: x,
            toughness: x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
