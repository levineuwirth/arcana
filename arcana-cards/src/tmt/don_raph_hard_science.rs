//! Don & Raph, Hard Science — `{1}{U/R}{U/R}` 2/4 Legendary Mutant Ninja Turtle.
//! Menace.
//! "Whenever Don & Raph attack, the next noncreature spell you cast this turn
//! has affinity for artifacts."
//!
//! The attack trigger grants a cost-reduction rider (affinity for artifacts)
//! to your next noncreature spell this turn — a next-cast cost-reduction rider
//! is not expressible with the demonstrated effect API, so the trigger's
//! effect is GAP'd while the trigger structure is preserved.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Don & Raph, Hard Science");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U/R}{U/R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: grant_affinity_rider,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn grant_affinity_rider(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
    // GAP: "the next noncreature spell you cast this turn has affinity for
    // artifacts" — a next-cast cost-reduction rider is not expressible with
    // the demonstrated effect API.
}
