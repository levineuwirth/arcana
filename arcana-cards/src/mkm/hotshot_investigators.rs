//! Hotshot Investigators — `{5}{U}` 4/4 blue Vedalken Detective.
//! "When this creature enters, return up to one other target creature to its owner's hand.
//! If you controlled it, investigate."
//!
//! GAP: "if you controlled it, investigate" — conditional investigate not expressible.
//! Using ReturnToHand for the main effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hotshot Investigators");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(detective);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn on_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you controlled it, investigate" — conditional investigate not expressible.
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else { return Vec::new(); };
    vec![Effect::ReturnToHand { target: *id }]
}
