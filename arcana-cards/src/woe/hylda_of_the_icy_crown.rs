//! Hylda of the Icy Crown — `{2}{W}{U}` 3/4 Legendary Human Warlock.
//! "Whenever you tap an untapped creature an opponent controls, you may pay
//!  {1}. When you do, choose one —
//!   • Create a 4/4 white and blue Elemental creature token.
//!   • Put a +1/+1 counter on each creature you control.
//!   • Scry 2, then draw a card."
//!
//! The trigger condition ("whenever you tap an untapped creature an opponent
//! controls") maps to `BecomesTapped { filter }`. Its payload is GAP'd:
//! * GAP: the "you may pay {1}. When you do, …" reflexive-payment +
//!   "choose one —" MODAL payload is not expressible on a triggered ability.
//!   `ModalSpec`/`dispatch_modal_effect` are SpellAbilityDef-only, the modes
//!   carry their own targets/choices a flat trigger effect fn can't post, and
//!   the reflexive "When you do" sub-trigger has no primitive. (`Scry` from
//!   Scryfall's keyword list is mode-3 reminder text, not a creature keyword.)

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
    let name = reg.interner_mut().intern("Hylda of the Icy Crown");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::BecomesTapped {
                filter: ObjectFilter::creature()
                    .untapped_only()
                    .controlled_by(ControllerConstraint::Opponent),
            },
            intervening_if: None,
            effect: hylda_payoff,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn hylda_payoff(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may pay {1}. When you do, choose one — …" — reflexive-payment
    // modal payload not expressible on a triggered ability (modal machinery
    // is SpellAbilityDef-only; reflexive "When you do" sub-trigger unmodeled).
    Vec::new()
}
