//! Daxos, Blessed by the Sun — `{W}{W}` 2/* Legendary Enchantment Creature — Demigod.
//! "Daxos's toughness is equal to your devotion to white." (CDA — GAP)
//! "Whenever another creature you control enters or dies, you gain 1 life."

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
    let name = reg.interner_mut().intern("Daxos, Blessed by the Sun");
    let demigod = reg.interner_mut().intern("Demigod");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demigod);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        // `*` toughness: GAP — the characteristic-defining ability "toughness
        // equal to your devotion to white" is not an expressible static, so
        // the `*` is marked but not computed.
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: gain_one_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: gain_one_life_on_death,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_one_life(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "another creature" — exclude Daxos itself entering.
    if let Some(id) = trig.entering_object() {
        if id == trig.source {
            return Vec::new();
        }
    }
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 1,
    }]
}

fn gain_one_life_on_death(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "another creature" — exclude Daxos itself dying.
    if let Some(id) = trig.dying_object() {
        if id == trig.source {
            return Vec::new();
        }
    }
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 1,
    }]
}
