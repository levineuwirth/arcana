//! Seraph of New Capenna // Seraph of New Phyrexia
//!
//! Front face: {2}{W} Creature — Angel Soldier 2/2, Flying. Activated ability:
//! {4}{B/P}: Transform this creature. Sorcery speed.
//! ({B/P} can be paid with either {B} or 2 life.)
//!
//! Back face: Creature — Phyrexian Angel, Flying. Whenever this creature attacks,
//! you may sacrifice another creature or artifact. If you do, this creature gets
//! +2/+1 until end of turn.
//!
//! The back-face attack trigger is wired as a face-gated trigger (face 1) via
//! Effect::OptionalPayment { Sacrifice(CreatureOrArtifact) → pump source +2/+1
//! EOT }. Caveat: SacrificeFilter can't encode "another" — the source itself is
//! offered as a legal sacrifice (minor over-inclusion).

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::{Effect, KeywordAbility};
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seraph of New Capenna");
    let angel_sub = reg.interner_mut().intern("Angel");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel_sub);
    subtypes.0.insert(soldier_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Seraph of New Phyrexia");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let angel_back_sub = reg.interner_mut().intern("Angel");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(angel_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: {4}{B/P}: Transform this creature. Activate only as a sorcery.
            // ({B/P} parses as a Phyrexian colored pip — pay {B} or 2 life.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{B/P}: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{B/P}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: front_transform,
            })
            // Back face (face 1): "Whenever this creature attacks, you may
            // sacrifice another creature or artifact. If you do, this creature
            // gets +2/+1 until end of turn."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: back_attacks_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1),
    )
}

fn front_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn back_attacks_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may sacrifice another creature or artifact. If you do, this creature
    // gets +2/+1 until end of turn."
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::CreatureOrArtifact),
        then: Box::new(Effect::Pump {
            target: trig.source,
            power: 2,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
        else_effect: None,
    }]
}
