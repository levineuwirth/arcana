//! Soul Charmer — `{2}{W}` 2/2 white creature. "Whenever this creature deals
//! combat damage to a creature, you gain 2 life unless that creature's
//! controller pays {2}."
//!
//! GAP: effect — "unless opponent pays {2}" optional payment rider; no
//! Effect variant for conditional opponent-payment gate. Emitting life gain
//! unconditionally.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul Charmer");
    let human = reg.interner_mut().intern("Human");
    let rebel = reg.interner_mut().intern("Rebel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rebel);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Creature,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn combat_damage_gain_life(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: effect — "unless opponent pays {2}"; no optional-payment Effect variant
    vec![Effect::GainLife { player: trig.controller, amount: 2 }]
}
