//! The Pride of Hull Clade — `{10}{G}` 2/15 Legendary Crocodile Elk Turtle.
//! This spell costs {X} less to cast, where X is the total toughness of
//! creatures you control.
//! Defender.
//! {2}{U}{U}: Until end of turn, target creature you control gets +1/+0,
//! gains "Whenever this creature deals combat damage to a player, draw
//! cards equal to its toughness," and can attack as though it didn't have
//! defender.
//!
//! Defender is a base keyword. The "{X} less to cast" cost-reduction
//! static is GAP'd (no scaling cost-reduction expressible). The activated
//! ability pumps the target (+1/+0) and grants it a combat-damage
//! triggered ability that draws cards equal to its toughness (computed at
//! resolution). The "can attack as though it didn't have defender" rider
//! is GAP'd (no defender-suppression primitive).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Pride of Hull Clade");
    let crocodile = reg.interner_mut().intern("Crocodile");
    let elk = reg.interner_mut().intern("Elk");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crocodile);
    subtypes.0.insert(elk);
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{10}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(15)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: "This spell costs {X} less to cast, where X is the total
    // toughness of creatures you control." — no scaling cost reduction.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{U}{U}: Until end of turn, target creature you control gets +1/+0, gains \"Whenever this creature deals combat damage to a player, draw cards equal to its toughness,\" and can attack as though it didn't have defender.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{U}{U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: empower_creature,
        }),
    )
}

fn empower_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "can attack as though it didn't have defender" — no
    // defender-suppression primitive in the demonstrated API.
    vec![
        Effect::Pump {
            target: *id,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::GrantTriggeredAbility {
            target: *id,
            ability: Box::new(TriggeredAbilityDef {
                id: GRANTED_TRIGGER_ID_BASE + 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: draw_equal_to_toughness,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
            duration: Duration::EndOfTurn,
        },
    ]
}

fn draw_equal_to_toughness(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::toughness_of(state, trig.source).max(0) as u32;
    vec![Effect::DrawCards {
        player: trig.controller,
        count: n,
    }]
}
