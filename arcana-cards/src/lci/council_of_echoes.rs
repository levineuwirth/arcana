//! Council of Echoes — `{4}{U}{U}` 4/4 blue Creature — Spirit Advisor.
//!
//! Flying
//! Descend 4 — When this creature enters, if there are four or more
//!   permanent cards in your graveyard, return up to one target nonland
//!   permanent other than this creature to its owner's hand.
//!
//! Decomposition: Flying → `keywords`; the Descend-gated ETB → one
//! `TriggeredAbilityDef`. "Descend 4" is the intervening-if (four or more
//! cards in your graveyard); the trigger returns up to one target nonland
//! permanent to its owner's hand. Descend is the gate, not a separate
//! emitted keyword. The "permanent cards" nuance of Descend is
//! approximated by `conditions::graveyard_at_least` (counts all graveyard
//! cards) — a documented fidelity gap. "Other than this creature" is
//! enforced in the resolver.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Council of Echoes");
    let spirit = reg.interner_mut().intern("Spirit");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: Some(if_descend_4),
            effect: bounce_nonland,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn if_descend_4(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::graveyard_at_least(s, you, 4)
}

fn bounce_nonland(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    if *id == trig.source {
        return Vec::new();
    }
    vec![Effect::ReturnToHand { target: *id }]
}
