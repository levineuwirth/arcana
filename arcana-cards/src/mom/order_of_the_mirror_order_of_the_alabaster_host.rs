//! Order of the Mirror // Order of the Alabaster Host — `{1}{U}` Human Knight 2/1 (front).
//! Front face:
//!   {3}{W/P}: Transform this creature. Activate only as a sorcery.
//!   ({W/P} can be paid with either {W} or 2 life.)
//! Back face (Order of the Alabaster Host) — Phyrexian Knight:
//!   Whenever this creature becomes blocked by a creature, the blocking creature
//!   gets -1/-1 until end of turn.
//!
//! GAP: {W/P} hybrid phyrexian mana cost — ManaCost::parse handles the symbol but the
//!      "or 2 life" alternative payment is not a separate engine gate; modeled as mana only.
//! GAP: back-face-only triggered ability (blocker -1/-1) not auto-installed on transform.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;

use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Order of the Mirror");
    let human_sub = reg.interner_mut().intern("Human");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(knight_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Order of the Alabaster Host — Phyrexian Knight
    let back_name = reg.interner_mut().intern("Order of the Alabaster Host");
    let back_phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let back_knight_sub = reg.interner_mut().intern("Knight");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_phyrexian_sub);
    back_subtypes.0.insert(back_knight_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            // Back face is white per Phyrexian transformation
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            supertypes: SupertypeSet::default(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face activated ability: {3}{W/P} — transform (sorcery speed).
            // GAP: {W/P} Phyrexian mana "or pay 2 life" alternate cost not separately modeled.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{W/P}: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{W/P}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: front_transform,
            })
            // Back face: "whenever this creature becomes blocked by a creature, that creature
            // gets -1/-1 until end of turn."
            // GAP: back-face-only triggered ability not auto-installed on transform.
            // Modeled on the definition anyway so it fires from the battlefield.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: blocker_debuff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn front_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

/// Whenever this creature becomes blocked, the first blocker gets -1/-1 until end of turn.
fn blocker_debuff(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(blocker_id) = trig.other_combatant() else { return Vec::new(); };
    vec![Effect::Pump {
        target: blocker_id,
        power: -1,
        toughness: -1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
