//! Sanwell, Avenger Ace — `{1}{W}` 3/1 Legendary white Human Pilot.
//! As long as an artifact creature you control is attacking, prevent all
//! damage that would be dealt to Sanwell.
//! Whenever Sanwell becomes tapped, exile the top six cards of your library.
//! You may cast a Vehicle or artifact creature spell from among them. Then
//! put the rest on the bottom of your library in a random order.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sanwell, Avenger Ace");
    let human = reg.interner_mut().intern("Human");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pilot);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    // GAP: conditional static "prevent all damage to Sanwell while an artifact
    // creature you control is attacking" is a continuous prevention static,
    // not a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: impulse_six,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn impulse_six(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "exile the top six, you may cast a Vehicle/artifact-creature spell from
    // among them, rest to bottom random." Modeled as a 6-card impulse exile
    // (FIDELITY GAP: the Vehicle/artifact-creature cast restriction and the
    // single-cast limit are not enforced; ImpulseExile permits playing any of
    // the exiled cards this window).
    vec![Effect::ImpulseExile { player: trig.controller, count: 6 }]
}
