//! Invasion of Tarkir // Defiant Thundermaw — `{1}{R}` Battle — Siege with 6 defense counters.
//! Front: When this Siege enters, reveal any number of Dragon cards from your hand.
//!   When you do, deals X+2 damage to any other target, where X = cards revealed.
//! Back face: Defiant Thundermaw — Creature — Dragon with Flying, Trample.
//!   Whenever a Dragon you control attacks, it deals 2 damage to any target.
//!
//! # GAP
//! "Reveal any number of Dragon cards from your hand" — choosing cards from hand
//!   to reveal is not expressible; X is the count of revealed cards which requires
//!   player choice. The ETB damage trigger is GAP'd (Vec::new()) since the reveal
//!   count cannot be computed.
//! Back face triggered ability "whenever a Dragon you control attacks" is a
//!   back-face-only triggered ability; emitted here but will always be active
//!   (engine doesn't gate by face). // GAP: back-face-only triggered ability not modeled.
//! Scryfall keyword Transform: the defeat→cast-back-face is not auto-wired.
//!   // GAP: defeat→cast-back-face not auto-wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Tarkir");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Defiant Thundermaw — Dragon creature, Flying + Trample
    let back_name = reg.interner_mut().intern("Defiant Thundermaw");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(dragon_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Pre-intern Dragon for the attack trigger filter
    let _dragon_sub2 = reg.interner_mut().intern("Dragon");

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 6,
            })
            // GAP: ETB "reveal Dragon cards, deal X+2 damage" — reveal count not expressible.
            // GAP: back-face-only triggered ability not modeled (always active here).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // Whenever a Dragon you control attacks — back-face-only in rules, but active always here.
                // GAP: back-face-only triggered ability not modeled.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(arcana_core::targets::ControllerConstraint::You),
                },
                intervening_if: None,
                effect: dragon_attacks_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }),
    )
}

fn dragon_attacks_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Deals 2 damage to any target.
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            arcana_core::targets::ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            arcana_core::targets::ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: dt,
        amount: 2,
    }]
}
