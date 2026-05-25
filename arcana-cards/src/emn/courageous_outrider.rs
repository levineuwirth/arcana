//! Courageous Outrider — `{3}{W}` 3/4 white Creature — Human Scout.
//! "When this creature enters, look at the top four cards of your
//! library. You may reveal a Human card from among them and put it
//! into your hand. Put the rest on the bottom of your library in
//! any order."
//! GAP: effect — "look at top 4, reveal a Human and put in hand, rest
//! to bottom in any order" has no catalog equivalent. Using
//! TutorToHand as rough approximation (no top-4 restriction).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Courageous Outrider");
    let human = reg.interner_mut().intern("Human");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_human,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor_human(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: effect — "look at top 4, choose Human, put rest on bottom
    // in any order" has no catalog equivalent. Using TutorToHand with
    // Human filter as rough approximation.
    let human_filter = script::subtype_filter(reg, "Human");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: human_filter,
        reveal: true,
    }]
}
