//! Excava, the Risen Past — `{2}{R}{W}` Legendary 3/3 Spirit Horse with
//! Flying and Haste.
//!
//! Oracle:
//! * Flying, haste.
//! * Whenever Excava attacks, return up to one target artifact, creature,
//!   or non-Aura enchantment card with mana value 3 or less from your
//!   graveyard to the battlefield with a finality counter on it. It's a
//!   1/1 Spirit creature with flying in addition to its other types.
//!   - Wired: the targeted graveyard return + the finality counter.
//!   - GAP: the "becomes a 1/1 Spirit with flying" continuous overlay is
//!     not expressible as a side-effect of the return; the non-Aura
//!     refinement is also approximated (we filter to artifact/creature/
//!     enchantment by mv only).

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Excava, the Risen Past");
    let spirit = reg.interner_mut().intern("Spirit");
    let horse = reg.interner_mut().intern("Horse");
    // Pre-intern the finality counter name so the resolver's lookup succeeds.
    let _finality = reg.interner_mut().intern("finality");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(horse);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: reanimate_from_graveyard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new()
                        .with_types_any(TypeLine(
                            TypeLine::ARTIFACT | TypeLine::CREATURE | TypeLine::ENCHANTMENT,
                        ))
                        .with_max_cmc(3),
                },
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn reanimate_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let finality = reg
        .interner()
        .lookup("finality")
        .map(CounterKind::Named);
    let mut effects = vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }];
    if let Some(kind) = finality {
        effects.push(Effect::AddCounters {
            target: *id,
            kind,
            count: 1,
        });
    }
    // GAP: "becomes a 1/1 Spirit creature with flying in addition to its
    // other types" continuous overlay on the returned card is unmodeled.
    effects
}
