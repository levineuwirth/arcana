//! Halana, Kessig Ranger — `{3}{G}` 3/4 Legendary Human Archer Ranger.
//!
//! Oracle:
//! * Reach
//! * Whenever another creature you control enters, you may pay {2}. When you
//!   do, that creature deals damage equal to its power to target creature.
//! * Partner
//!
//! Reach is a base characteristic. The enters trigger fires on another
//! creature you control entering, but its effect is a reflexive
//! may-pay-then-target ("you may pay {2}. When you do, [the entering
//! creature] deals damage equal to its power to target creature") that the
//! demonstrated primitives cannot express (OptionalPayment cannot choose a
//! creature target at the moment payment resolves, nor reference the
//! entering creature's power as the damage amount), so the effect is GAP'd.
//! Partner is not a KeywordAbility variant and is GAP'd.

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
    let name = reg.interner_mut().intern("Halana, Kessig Ranger");
    let human = reg.interner_mut().intern("Human");
    let archer = reg.interner_mut().intern("Archer");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(archer);
    subtypes.0.insert(ranger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Partner is not a KeywordAbility variant.
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: reflexive_pay_then_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn reflexive_pay_then_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reflexive "you may pay {2}. When you do, that creature deals
    // damage equal to its power to target creature." — OptionalPayment's
    // `then` cannot choose a creature target at payment-resolution time, and
    // the damage amount references the entering creature's power, which is
    // not expressible with the demonstrated primitives.
    Vec::new()
}
