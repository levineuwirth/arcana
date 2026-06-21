//! Tilonalli's Skinshifter — `{2}{R}` 0/1 Human Shaman with Haste.
//! "Whenever this creature attacks, it becomes a copy of another target
//! nonlegendary attacking creature until end of turn."
//!
//! Haste is an engine keyword. The attack trigger targets another
//! nonlegendary attacking creature.
//!
//! GAP: "it becomes a copy of …" makes THIS creature become a copy of
//! the target. The only copy primitive is `Effect::CopyPermanent`, which
//! MINTS A NEW TOKEN copy rather than transforming an existing object
//! into a copy, so the effect cannot be expressed faithfully and the
//! resolution body is GAP'd. The trigger + target are still emitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tilonalli's Skinshifter");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    // "another nonlegendary attacking creature" — restrict to nonlegendary
    // creatures (the "attacking" + "another" restrictions are documented
    // over-fire: no attacking-status filter is available here).
    let target_filter = ObjectFilter::creature()
        .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY));

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: become_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(target_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn become_copy(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "it becomes a copy of target creature" — no self-becomes-a-copy
    // primitive. `CopyPermanent` mints a token instead of transforming this
    // object, so the effect is GAP'd.
    Vec::new()
}
