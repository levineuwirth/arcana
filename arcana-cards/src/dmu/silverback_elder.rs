//! Silverback Elder — `{2}{G}{G}{G}` 5/7 Ape Shaman.
//! "Whenever you cast a creature spell, choose one —
//!  • Destroy target artifact or enchantment.
//!  • Look at the top five cards of your library. You may put a land card
//!    from among them onto the battlefield tapped. Put the rest on the
//!    bottom of your library in a random order.
//!  • You gain 4 life."
//!
//! The trigger (you cast a creature spell) is expressible, but a "choose
//! one" modal on a TRIGGERED ability has no machinery in this card class
//! (modal dispatch is only available for spell abilities), so the
//! choice-bearing effect body is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Silverback Elder");
    let ape = reg.interner_mut().intern("Ape");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ape);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cast_creature_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn cast_creature_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose one" modal on a triggered ability is not expressible —
    // triggered abilities have no modal/choose-one machinery in this card
    // class (only spell abilities carry `modal`).
    Vec::new()
}
