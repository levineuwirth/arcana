//! Staunch Crewmate — `{1}{U}` 2/1 blue Human Pirate. "When this creature
//! enters, look at the top four cards of your library. You may reveal an
//! artifact or Pirate card from among them and put it into your hand. Put
//! the rest on the bottom of your library in a random order."
//!
//! GAP: look-at-4 with conditional reveal (artifact or Pirate) not expressible;
//! using TutorToHand as best-effort for the "reveal and put in hand" portion.

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
    let name = reg.interner_mut().intern("Staunch Crewmate");
    let human = reg.interner_mut().intern("Human");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pirate);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: look at top 4 and conditionally put artifact or Pirate to hand;
    // using TutorToHand (artifact or Pirate) as best-effort
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::new()
            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
            .controlled_by(ControllerConstraint::You),
        reveal: true,
    }]
}
