//! Invasion of Kamigawa // Rooftop Saboteurs
//!
//! Front face: `{3}{U}` Battle — Siege with 4 defense counters.
//! When this Siege enters, tap target artifact or creature an opponent controls
//! and put a stun counter on it.
//!
//! Back face: Creature — Moonfolk Ninja, Flying.
//! Whenever this creature deals combat damage to a player or battle, draw a card.
//!
//! GAP: defeat→cast-back-face not auto-wired (CR 310.11 deferred).
//! GAP: Ninja keyword (Ninjutsu) not in KeywordAbility enum.
//! GAP: Back-face-only triggered ability (combat damage draw) not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Kamigawa");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Rooftop Saboteurs — Creature — Moonfolk Ninja, Flying
    let back_name = reg.interner_mut().intern("Rooftop Saboteurs");
    let moonfolk_sub = reg.interner_mut().intern("Moonfolk");
    let ninja_sub = reg.interner_mut().intern("Ninja");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(moonfolk_sub);
    back_subtypes.0.insert(ninja_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 4,
            })
            .with_transform_back(back)
            // ETB: tap target artifact or creature an opponent controls, put a stun counter
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tap_stun,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
        // GAP: back-face-only triggered ability not modeled —
        //      "Whenever this creature deals combat damage to a player or battle, draw a card"
    )
}

fn etb_tap_stun(
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
    vec![
        Effect::Tap { target: *id },
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::Stun,
            count: 1,
        },
    ]
}
