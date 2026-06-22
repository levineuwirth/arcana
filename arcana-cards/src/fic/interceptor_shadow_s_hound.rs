//! Interceptor, Shadow's Hound — `{2}{B}{B}` 4/3 Legendary Dog.
//!
//! * Menace.
//! * "Assassins you control have menace." — a static keyword-granting
//!   continuous ability over other creatures you control; not
//!   expressible with the demonstrated primitives. GAP'd.
//! * "Whenever you attack with one or more legendary creatures, you may
//!   pay {2}{B}. If you do, return this card from your graveyard to the
//!   battlefield tapped and attacking." — a graveyard-zoned trigger
//!   whose payoff returns this card to the battlefield tapped and
//!   attacking; there is no effect that returns a card from the
//!   graveyard to the battlefield tapped-and-attacking. GAP'd: the
//!   trigger shell (approximated by "a legendary creature you control
//!   attacks", watched from the graveyard) is emitted with an empty
//!   effect body.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Interceptor, Shadow's Hound");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: "Assassins you control have menace." — static keyword grant
    // over other creatures, not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
            },
            intervening_if: None,
            effect: graveyard_return,
            trigger_zones: vec![Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn graveyard_return(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may pay {2}{B}; return this from your graveyard to the
    // battlefield tapped and attacking." No effect returns a card from
    // the graveyard to the battlefield tapped-and-attacking.
    Vec::new()
}
