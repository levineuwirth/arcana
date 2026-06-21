//! Hare Apparent — `{1}{W}` 2/2 white Rabbit Noble.
//! "When this creature enters, create a number of 1/1 white Rabbit
//! creature tokens equal to the number of other creatures you control
//! named Hare Apparent." (The "a deck can have any number of cards
//! named Hare Apparent" line is a deckbuilding rule, not a game
//! ability — nothing to wire.)

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Hare Apparent");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_rabbits,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_rabbits(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // Count OTHER creatures you control named "Hare Apparent". The
    // count_matching helper counts all matching permanents you control;
    // the source itself matches by name, so subtract 1 for "other".
    let nm = reg.interner().lookup("Hare Apparent");
    let filter = ObjectFilter {
        name: nm,
        ..ObjectFilter::creature().controlled_by(ControllerConstraint::You)
    };
    let total = script::count_matching(state, &filter, trig.controller);
    let n = total.saturating_sub(1);
    let rabbit = reg.interner().lookup("Rabbit").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    let token = TokenDefinition {
        name: rabbit,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        })
        .collect()
}
