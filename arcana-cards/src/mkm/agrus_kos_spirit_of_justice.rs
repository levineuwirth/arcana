//! Agrus Kos, Spirit of Justice — `{2}{R}{W}` 2/4 Legendary Spirit
//! Detective with Double strike and Vigilance.
//!
//! Oracle:
//! * Double strike, vigilance — keyword line. (Suspect appears in
//!   Scryfall's keyword list as the named action used by the ability
//!   below, not as a static keyword on the type line.)
//! * Whenever Agrus Kos enters or attacks, choose up to one target
//!   creature. If it's suspected, exile it. Otherwise, suspect it.
//!
//! "Enters or attacks" is two `TriggerCondition`s, so it is decomposed
//! into a SelfEntersBattlefield trigger and a SelfAttacks trigger sharing
//! one resolver, each with an up-to-one target creature. The conditional
//! body emits `Effect::Suspect` ("otherwise, suspect it" — the default
//! branch); the "if it's suspected, exile it" branch is GAP'd because no
//! `Condition` variant can read the chosen target's suspected flag.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Agrus Kos, Spirit of Justice");
    let spirit = reg.interner_mut().intern("Spirit");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::DoubleStrike, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: suspect_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![up_to_one_creature()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: suspect_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![up_to_one_creature()],
            }),
    )
}

fn up_to_one_creature() -> TargetRequirement {
    TargetRequirement {
        filter: TargetFilter::Permanent(ObjectFilter::creature()),
        count: TargetCount::UpTo(1),
        controller: None,
    }
}

fn suspect_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "If it's suspected, exile it" — no Condition variant can read the
    // chosen target's suspected flag, so only the "otherwise, suspect it"
    // branch is emitted.
    vec![Effect::Suspect { target: *id }]
}
