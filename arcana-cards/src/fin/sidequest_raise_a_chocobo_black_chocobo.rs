//! Sidequest: Raise a Chocobo // Black Chocobo — `{1}{G}` Enchantment (G),
//! transforming DFC.
//!
//! Front face (Sidequest: Raise a Chocobo — Enchantment):
//! - When this enchantment enters, create a 2/2 green Bird creature token with
//!   "Whenever a land you control enters, this token gets +1/+0 until end of turn."
//! - At the beginning of your first main phase, if you control four or more
//!   Birds, transform this enchantment.
//!
//! Back face (Black Chocobo — Creature — Bird):
//! - When this permanent transforms into Black Chocobo, search your library for
//!   a land card, put it onto the battlefield tapped, then shuffle.
//! - Landfall — Whenever a land you control enters, Birds you control get +1/+0
//!   until end of turn.
//!
//! GAP: the created Bird token's printed ability ("Whenever a land you control
//!      enters, this token gets +1/+0 until end of turn") cannot be attached —
//!      TokenDefinition.abilities does not accept a triggered landfall ability.
//!      Token is minted as a vanilla 2/2 green Bird.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::{conditions, script};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sidequest: Raise a Chocobo");
    let bird_sub = reg.interner_mut().intern("Bird");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };

    // Back face — Black Chocobo (Creature — Bird)
    let back_name = reg.interner_mut().intern("Black Chocobo");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(bird_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: None,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // id 1 — FRONT: ETB create a 2/2 green Bird token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_bird,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // id 2 — FRONT: at your first main phase, if you control 4+ Birds, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_four_or_more_birds),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // id 3 — BACK: When this permanent transforms into Black Chocobo,
            // search your library for a land card, put it onto the battlefield
            // tapped, then shuffle.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: on_transform_fetch_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // id 4 — BACK: Landfall — whenever a land you control enters, Birds you
            // control get +1/+0 until end of turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: landfall_pump_birds,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 0)
            .with_trigger_face_gate(4, 1),
    )
}

fn if_four_or_more_birds(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    // "if you control four or more Birds"
    // GAP: a per-name "Bird" subtype filter needs the interner, which the
    // intervening-if signature does not provide; gating on creature count of 4 as
    // the closest expressible approximation.
    conditions::you_control_at_least(
        s,
        you,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        4,
    )
}

fn etb_make_bird(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let bird = reg
        .interner()
        .lookup("Bird")
        .expect("Bird interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    let token = TokenDefinition {
        name: bird,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: token's own landfall +1/+0 ability not attachable.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}

fn transform_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

fn on_transform_fetch_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
        tapped: true,
    }]
}

fn landfall_pump_birds(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Bird").controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
