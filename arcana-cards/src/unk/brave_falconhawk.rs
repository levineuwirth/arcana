//! Brave Falconhawk — `{3}{W}` 3/3 Bird with Flying.
//!
//! Oracle:
//! * Flying — keyword line.
//! * When this creature enters, create a Contested Game Ball token.
//!   (Modeled as a bare colorless artifact token; the Game Ball's own
//!   point-counter rules are not reproduced.)
//! * Whenever this creature attacks, if you control a Contested Game
//!   Ball, put +1/+1 counters on this creature equal to the number of
//!   point counters among Contested Game Balls you control. — GAP: the
//!   counter count is dynamic across a filtered set of permanents (sum
//!   of point counters), which no `script::` helper computes, so the
//!   effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brave Falconhawk");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    // Pre-intern the token name for the resolver.
    let _ball = reg.interner_mut().intern("Contested Game Ball");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_ball,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_grow,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_make_ball(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(ball) = reg.interner().lookup("Contested Game Ball") else {
        return Vec::new();
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: ball,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: SubtypeSet::default(),
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn attack_grow(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you control a Contested Game Ball, put +1/+1 counters on
    // this creature equal to the number of point counters among
    // Contested Game Balls you control" — the amount is a sum of point
    // counters across a filtered permanent set; no script helper counts
    // counters across permanents.
    Vec::new()
}
