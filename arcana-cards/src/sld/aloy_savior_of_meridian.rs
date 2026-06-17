//! Aloy, Savior of Meridian — `{3}{G}{U}` 3/5 Legendary Human Warrior
//! with Vigilance and Reach.
//! "In You, All Things Are Possible — Whenever one or more artifact
//! creatures you control attack, discover X, where X is the greatest power
//! among them."
//!
//! Vigilance and Reach are wired as keywords (Discover is an Effect, not a
//! KeywordAbility). The attack trigger is wired (CreatureAttacks filtered to
//! artifact creatures you control), but its body is GAP'd: Effect::Discover
//! takes a FIXED mana_value, and there is no script helper for "greatest
//! power among the attacking creatures", so the dynamic X is inexpressible.

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
    let name = reg.interner_mut().intern("Aloy, Savior of Meridian");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature()
                    .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
                    .controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: discover_x,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn discover_x(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "discover X, where X is the greatest power among them" —
    // Effect::Discover requires a fixed mana_value and no script helper
    // computes the greatest power among the attacking creatures.
    Vec::new()
}
