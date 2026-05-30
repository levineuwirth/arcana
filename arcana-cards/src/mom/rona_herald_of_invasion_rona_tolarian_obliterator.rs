//! Rona, Herald of Invasion // Rona, Tolarian Obliterator — `{1}{U}` Legendary Human Wizard 1/3 (front).
//! Whenever you cast a legendary spell, untap Rona.
//! {T}: Draw a card, then discard a card.
//! {5}{B/P}: Transform Rona. Activate only as a sorcery.
//! Back face (Rona, Tolarian Obliterator): Legendary Phyrexian Wizard with Trample.
//! Whenever a source deals damage to Rona, that source's controller exiles a card
//! from their hand at random. If it's a land card, you may put it onto the battlefield
//! under your control. Otherwise, you may cast it without paying its mana cost.
//!
//! GAP: {B/P} hybrid-Phyrexian mana cost in transform activation; parsed as best-effort.
//! GAP: back-face triggered ability "whenever a source deals damage to Rona, that source's
//! controller exiles a card from their hand at random; if it's a land, put it onto the
//! battlefield; otherwise cast it for free" — no TriggerCondition for "source deals damage
//! to this permanent" (SelfIsDealtDamage fires on our permanent, but we need the source's
//! controller; also the conditional land-vs-spell branching on a randomly exiled card is not
//! expressible). Back-face triggered ability omitted.
//! GAP: back-face-only triggered ability not auto-installed on transform.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rona, Herald of Invasion");
    let human_sub = reg.interner_mut().intern("Human");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Rona, Tolarian Obliterator");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let back_wizard_sub = reg.interner_mut().intern("Wizard");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(back_wizard_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Whenever you cast a legendary spell, untap Rona.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: untap_rona,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {T}: Draw a card, then discard a card.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Draw a card, then discard a card.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0),
                effect: tap_draw_discard,
            })
            // {5}{B/P}: Transform Rona. Sorcery speed only.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{B/P}: Transform Rona. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{B/P}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_rona,
            }),
        // GAP: back-face triggered ability not auto-installed on transform.
    )
}

fn untap_rona(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Untap { target: trig.source }]
}

fn tap_draw_discard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::Discard { player: ctx.controller, count: 1, choice: DiscardChoice::ControllerChooses },
    ]
}

fn transform_rona(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
