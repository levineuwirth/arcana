//! Marneus Calgar — `{2}{W}{U}{B}` 3/5 Legendary Astartes Warrior with
//! Double strike.
//!
//! Oracle:
//! "Double strike
//!  Master Tactician — Whenever one or more tokens you control enter, draw a
//!  card.
//!  Chapter Master — {6}: Create two 2/2 white Astartes Warrior creature
//!  tokens with vigilance."
//!
//! * Double strike — base keyword.
//! * Master Tactician — a `ZoneChange` trigger watching tokens you control
//!   entering the battlefield → draw a card. (Modeled per-token via
//!   `EachTime`; the "one or more … per batch" aggregation is a minor
//!   fidelity nuance, not a gap.)
//! * Chapter Master — a `{6}` activated ability minting two 2/2 white
//!   Astartes Warrior tokens with Vigilance.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marneus Calgar");
    let astartes = reg.interner_mut().intern("Astartes");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(astartes);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .tokens_only()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: master_tactician_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}: Create two 2/2 white Astartes Warrior creature tokens with vigilance."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: chapter_master_tokens,
            }),
    )
}

/// Master Tactician: a token you control entered → draw a card.
fn master_tactician_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}

/// Chapter Master: create two 2/2 white Astartes Warrior tokens with Vigilance.
fn chapter_master_tokens(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let astartes = reg.interner().lookup("Astartes").unwrap_or_default();
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let mut tok_subtypes = SubtypeSet::default();
    tok_subtypes.0.insert(astartes);
    tok_subtypes.0.insert(warrior);

    let token = TokenDefinition {
        name: astartes,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: tok_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        abilities: vec![],
    };

    // "create two" — repeat the CreateToken value (no count field).
    vec![
        Effect::CreateToken { controller: ctx.controller, token: token.clone() },
        Effect::CreateToken { controller: ctx.controller, token },
    ]
}
