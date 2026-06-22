//! Sidar Jabari of Zhalfir — `{1}{W}{U}{B}` 4/3 Legendary Human Knight.
//!
//! Oracle:
//! * Eminence — "Whenever you attack with one or more Knights, if Sidar
//!   Jabari is in the command zone or on the battlefield, draw a card,
//!   then discard a card." The Eminence intervening-if (source in command
//!   zone OR on battlefield) is not expressible with the available
//!   `conditions::` predicates, and Eminence's command-zone activity is
//!   not modeled, so this clause's gate is GAP'd; we fire the
//!   draw-then-discard on a Knight-you-control attack (per attacking
//!   Knight, a fidelity gap vs once-per-attack).
//! * Flying, first strike (keyword line).
//! * "Whenever Sidar Jabari deals combat damage to a player, return target
//!   Knight creature card from your graveyard to the battlefield."

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sidar Jabari of Zhalfir");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let knight_attacker_filter = script::subtype_filter(reg, "Knight")
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: Eminence intervening-if "if Sidar Jabari is in the
            // command zone or on the battlefield" — not expressible with
            // available conditions; command-zone activity unmodeled.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: knight_attacker_filter,
                },
                intervening_if: None,
                effect: loot_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: reanimate_knight,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: script::subtype_filter(reg, "Knight"),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn loot_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ])]
}

fn reanimate_knight(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
