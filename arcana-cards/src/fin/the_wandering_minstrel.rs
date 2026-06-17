//! The Wandering Minstrel — `{G}{U}` 1/3 legendary Human Bard.
//! "Lands you control enter untapped." (a static replacement — GAP'd.)
//! "The Minstrel's Ballad — At the beginning of combat on your turn, if
//!  you control five or more Towns, create a 2/2 Elemental creature
//!  token that's all colors."
//! "{3}{W}{U}{B}{R}{G}: Other creatures you control get +X/+X until end
//!  of turn, where X is the number of Towns you control." (the "other"
//!  exclusion of self is a fidelity gap — applies to all your creatures.)

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Wandering Minstrel");
    let human = reg.interner_mut().intern("Human");
    let bard = reg.interner_mut().intern("Bard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(bard);
    let _elemental = reg.interner_mut().intern("Elemental");
    let _town = reg.interner_mut().intern("Town");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "Lands you control enter untapped."
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_five_towns),
                effect: make_elemental,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{W}{U}{B}{R}{G}: Other creatures you control get +X/+X until end of turn, where X is the number of Towns you control."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{W}{U}{B}{R}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_creatures,
            }),
    )
}

fn if_five_towns(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    let towns = script::subtype_filter(reg, "Town").controlled_by(ControllerConstraint::You);
    arcana_core::conditions::you_control_at_least(s, you, &towns, 5)
}

fn make_elemental(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: elemental,
            colors: ColorSet::white()
                | ColorSet::blue()
                | ColorSet::black()
                | ColorSet::red()
                | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn pump_creatures(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::count_matching(
        state,
        &script::subtype_filter(reg, "Town").controlled_by(ControllerConstraint::You),
        ctx.controller,
    ) as i32;
    // GAP fidelity: "Other creatures" exclusion of source not expressed —
    // pumps all creatures you control via ForEach.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: arcana_core::objects::NULL_OBJECT_ID,
            power: x,
            toughness: x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
