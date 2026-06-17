//! Skyserpent Seeker — `{G}{U}` 1/1 Snake with Flying and Deathtouch.
//! "Exhaust — {4}: Reveal cards from the top of your library until you reveal
//! two land cards. Put those land cards onto the battlefield tapped and the
//! rest on the bottom of your library in a random order. Put a +1/+1 counter
//! on this creature."
//!
//! GAP: Exhaust ("activate only once" ever) has no dedicated cost field; the
//! activated ability is gated with `once_per_turn` as the closest fidelity
//! approximation.
//! GAP: "reveal until you reveal TWO land cards ... onto the battlefield
//! tapped" — RevealUntil finds only a single matching card, so the
//! two-land reveal-and-put is unexpressible and omitted; the +1/+1 counter
//! is implemented.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyserpent Seeker");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Exhaust — {4}: Reveal cards from the top of your library until you reveal two land cards. Put those land cards onto the battlefield tapped and the rest on the bottom of your library in a random order. Put a +1/+1 counter on this creature.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                once_per_turn: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: exhaust_effect,
        }),
    )
}

fn exhaust_effect(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "reveal until two land cards onto battlefield tapped" portion is
    // unexpressible (RevealUntil is single-match) — only the counter is emitted.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
