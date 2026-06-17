//! Ghalta and Mavren — `{3}{G}{G}{W}{W}` 12/12 Legendary Dinosaur Vampire
//! with Trample.
//! "Whenever you attack, choose one —
//!  • Create a tapped and attacking X/X green Dinosaur creature token with
//!    trample, where X is the greatest power among other attacking
//!    creatures.
//!  • Create X 1/1 white Vampire creature tokens with lifelink, where X is
//!    the number of other attacking creatures."
//!
//! Trample is wired. The attack trigger is wired with the closest available
//! condition (CreatureAttacks over your creatures), but its body is GAP'd:
//! the engine has no MODAL TRIGGERED ability (modal is spell-only), no
//! tapped-and-attacking token effect, and no dynamic-P/T or dynamic-count
//! token in the demonstrated API (TokenDefinition P/T must be Fixed).

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
    let name = reg.interner_mut().intern("Ghalta and Mavren");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(12)),
        toughness: Some(PtValue::Fixed(12)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: attack_modal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no modal triggered ability (modal is spell-only), no
    // tapped-and-attacking token effect, and no dynamic-P/T (X/X) or
    // dynamic-count token in the demonstrated API.
    Vec::new()
}
