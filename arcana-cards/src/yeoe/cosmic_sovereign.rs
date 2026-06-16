//! Cosmic Sovereign — `{3}{R}{R}` 3/4 Dragon.
//!
//! * Flying, Haste — base keyword characteristics.
//! * `{R}: This creature gets +1/+0 until end of turn.` — an
//!   activated pump ability (firebreathing).
//! * "At the beginning of combat on your turn, conjure a random
//!   creature card with mana value X onto the battlefield, where X is
//!   this creature's power. That creature gains haste. Exile it at the
//!   beginning of the next end step." — the trigger fires at the
//!   right time but its CONJURE effect (fabricate a random creature
//!   card with a given mana value from outside the game) is not
//!   expressible with the demonstrated Effect surface; the effect body
//!   is GAP'd while the trigger condition is still recorded.
//!
//! (Scryfall lists `Conjure` as a keyword, but it is not a usable
//! `KeywordAbility` variant for this class — omitted from the keyword
//! vec; its only appearance is inside the combat trigger above.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::layers::Duration;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cosmic Sovereign");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}: This creature gets +1/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: firebreathing,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: conjure_random_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn firebreathing(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn conjure_random_creature(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: conjure a random creature card with mana value equal to this
    // creature's power onto the battlefield (fabricating a card from
    // outside the game, with a haste grant + delayed end-step exile) is
    // not expressible with the demonstrated Effect surface.
    Vec::new()
}
