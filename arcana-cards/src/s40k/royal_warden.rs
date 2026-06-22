//! Royal Warden — `{3}{B}{B}` 3/2 black Artifact Creature — Necron.
//!
//! Oracle text:
//! * Phalanx Commander — When this creature enters, create two tapped
//!   2/2 black Necron Warrior artifact creature tokens.
//! * Unearth {3}{B} ({3}{B}: Return this card from your graveyard to
//!   the battlefield. It gains haste. Exile it at the beginning of the
//!   next end step or if it would leave the battlefield. Unearth only
//!   as a sorcery.)
//!
//! Implemented: the Phalanx Commander ETB trigger (creates the two
//! token bodies), and Unearth modeled as a graveyard-activated ability
//! (return to battlefield + gains haste + delayed exile at the next end
//! step).
//!
//! GAP: the created Warrior tokens entering TAPPED is not expressible
//! with the documented token effects (no plain "create tapped" variant;
//! only "create tapped AND attacking" exists), so they enter untapped.
//! GAP: "Exile it … if it would leave the battlefield" (the
//! leaves-the-battlefield replacement half of unearth) is omitted; only
//! the end-step exile is scheduled.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition, DelayedWhen, DelayedAction};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Royal Warden");
    let necron = reg.interner_mut().intern("Necron");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(necron);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // "Warrior" is interned here so the token resolver can recover it
    // via reg.interner().lookup("Warrior").
    let _ = warrior;
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: phalanx_commander,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Unearth {3}{B}".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: unearth,
            }),
    )
}

fn phalanx_commander(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: warrior,
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        // GAP: these should enter TAPPED — no plain "create tapped" effect.
        Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
}

fn unearth(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::ReturnFromGraveyardToBattlefield { target: ctx.source },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
        Effect::DelayedAction {
            source: ctx.source,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Exile,
        },
    ])]
}
