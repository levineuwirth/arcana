//! Surrak, Elusive Hunter — `{2}{G}` 4/3 Legendary Human Warrior with Trample.
//!
//! "This spell can't be countered.
//!  Trample
//!  Whenever a creature you control or a creature spell you control becomes the
//!  target of a spell or ability an opponent controls, draw a card."
//!
//! Trample is a base keyword. "This spell can't be countered" is a static cast
//! property with no expressible hook on this card shape — GAP'd.
//!
//! The becomes-target trigger can only be scoped to THIS creature
//! (`SelfBecomesTarget { caster: Opponent }`); the printed scope ("a creature
//! you control OR a creature spell you control") is broader than the available
//! self-only condition — that broadening is GAP'd, and the draw fires when
//! Surrak itself is targeted by an opponent.

// GAP (static): "This spell can't be countered." — no can't-be-countered hook
// on the creature card shape.
// GAP (trigger scope): the condition matches only Surrak itself becoming the
// target of an opponent's spell/ability; "a creature you control or a creature
// spell you control" (any of your creatures / creature spells) is not
// expressible — there is no your-creatures-become-target condition.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Surrak, Elusive Hunter");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBecomesTarget {
                caster: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: draw_a_card,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_a_card(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
