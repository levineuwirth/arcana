//! Xira, the Golden Sting — `{1}{B}{R}{G}` 3/3 Legendary Insect Assassin.
//! Flying, haste.
//! "Whenever Xira attacks, put an egg counter on another target creature
//! without an egg counter on it. When that creature dies, if it has an egg
//! counter on it, draw a card and create a 1/1 black Insect creature token
//! with flying."
//!
//! The attack trigger places an egg counter (a named counter) on a chosen
//! creature. The death-rider ("when that creature dies …") is a per-creature
//! delayed/granted trigger keyed on the egged creature; that linkage and its
//! intervening-if are not expressible with the demonstrated API, so it is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Xira, the Golden Sting");
    let insect = reg.interner_mut().intern("Insect");
    let assassin = reg.interner_mut().intern("Assassin");
    // Pre-intern the egg-counter name so the resolver's lookup succeeds.
    let _egg = reg.interner_mut().intern("egg");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: put_egg_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn put_egg_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // "another target creature without an egg counter on it" — the filter
    // (another / without-egg-counter) restrictions are best-effort; we place
    // the egg counter on the chosen creature.
    let Some(egg) = reg.interner().lookup("egg") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(egg),
        count: 1,
    }]
    // GAP: "When that creature dies, if it has an egg counter on it, draw a
    // card and create a 1/1 black Insect with flying" — a delayed death-rider
    // bound to the egged creature is not expressible with the demonstrated API.
}
