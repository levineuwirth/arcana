//! Invasion of Alara // Awaken the Maelstrom — `{W}{U}{B}{R}{G}` Battle — Siege.
//! Front (Siege, 6 defense counters): When this Siege enters, exile cards from
//! the top of your library until you exile two nonland cards with mana value 4
//! or less. You may cast one of those two cards without paying its mana cost.
//! Put one into your hand. Then put the other cards exiled this way on the
//! bottom of your library in a random order.
//! Back (Sorcery): Awaken the Maelstrom is all colors.
//! Target player draws two cards.
//! You may put an artifact card from your hand onto the battlefield.
//! Create a token that's a copy of a permanent you control.
//! Distribute three +1/+1 counters among one, two, or three creatures you control.
//! Destroy target permanent an opponent controls.
//!
//! The back-face sorcery's effects fire on the defeat-transform via the
//! `SelfTransforms{to_face:Some(1)}` trigger. The two target-bearing clauses
//! ("target player draws two cards" + "destroy target permanent an opponent
//! controls") are wired; the three choice-heavy riders remain GAP'd below.
//!
//! GAP: ETB "exile until two nonland ≤ mv4, cast one free, put one in hand" —
//!   RevealUntil finds the first matching card only (not two); no free-cast variant.
//!   Approximated as DigTopN with max_reveal-bounded filter (poor approximation);
//!   the full logic is inexpressible. Emitting a best-effort dig of 10 cards.
//! GAP: back-face "put an artifact card from your hand onto the battlefield" —
//!   no Effect variant for hand-to-battlefield (hand targets not supported).
//! GAP: back-face "create a token copy of a permanent you control (with choice)" —
//!   CopyPermanent requires a specific target id; no "you choose which" selection.
//! GAP: back-face "distribute three +1/+1 counters among creatures" — multi-target
//!   distribution (one/two/three creatures) not expressible; effect omitted.

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Alara");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Awaken the Maelstrom — Sorcery (all five colors)
    let back_name = reg.interner_mut().intern("Awaken the Maelstrom");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white()
                | ColorSet::blue()
                | ColorSet::black()
                | ColorSet::red()
                | ColorSet::green(),
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        },
        spell_ability: None, // back face cannot be cast
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 6,
            })
            .with_transform_back(back)
            // ETB trigger: approximate "dig until 2 nonland ≤ mv4, cast one free, keep one"
            // GAP: see module doc. Using RevealUntil to find the first nonland ≤ mv4.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back-face sorcery resolves on the defeat-transform: target player
            // draws two cards; destroy target permanent an opponent controls.
            // (The three choice-heavy riders are GAP'd — see module doc.)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: back_draw_and_destroy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    // Target player draws two cards.
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    // Destroy target permanent an opponent controls.
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new()
                                .controlled_by(ControllerConstraint::Opponent),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
            }),
    )
}

fn back_draw_and_destroy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    // First target: a player draws two cards.
    if let Some(TargetChoice::Player(p)) = trig.targets.targets.first() {
        effects.push(Effect::DrawCards { player: *p, count: 2 });
    }
    // Second target: destroy a permanent an opponent controls.
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.get(1) {
        effects.push(Effect::DestroyPermanent { target: *id });
    }
    effects
}

fn etb_dig(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should find two nonland cards with mv ≤ 4; cast one free, put one in hand.
    // RevealUntil finds only the FIRST match; free-cast is not expressible.
    // Approximated as RevealUntil for one nonland ≤ mv4 card into hand, rest on bottom.
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter: ObjectFilter::new()
            .without_types(TypeLine::LAND.into())
            .with_max_cmc(4),
        found_dest: RevealDest::Hand,
        rest: DigRest::BottomRandom,
        max_reveal: Some(10),
    }]
}
