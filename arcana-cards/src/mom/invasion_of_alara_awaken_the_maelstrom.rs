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
//! GAP: defeat→cast-back-face not auto-wired (CR 310.11 deferred).
//! GAP: ETB "exile until two nonland ≤ mv4, cast one free, put one in hand" —
//!   RevealUntil finds the first matching card only (not two); no free-cast variant.
//!   Approximated as DigTopN with max_reveal-bounded filter (poor approximation);
//!   the full logic is inexpressible. Emitting a best-effort dig of 10 cards.
//! GAP: back-face "put an artifact card from your hand onto the battlefield" —
//!   no Effect variant for hand-to-battlefield (hand targets not supported).
//! GAP: back-face "create a token copy of a permanent you control (with choice)" —
//!   CopyPermanent requires a specific target id; no "you choose which" selection.
//! GAP: back-face "distribute three +1/+1 counters among creatures" — multi-target
//!   distribution not expressible; effect omitted.
//! GAP: back-face "destroy target permanent an opponent controls" modeled faithfully.

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
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
            }),
    )
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
