//! Avatar Roku, Firebender — `{3}{R}{R}{R}` 6/6 Legendary Human Avatar.
//! "Whenever a player attacks, add six {R}. Until end of combat, you don't
//! lose this mana as steps end. {R}{R}{R}: Target creature gets +3/+0 until
//! end of turn."
//!
//! The attack trigger adds six red mana; the "you don't lose this mana as
//! steps end" mana-retention rider is not expressible with the demonstrated
//! primitives (recorded as a GAP — only the mana addition is modeled). The
//! pump activated ability is fully expressed.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Avatar Roku, Firebender");
    let human = reg.interner_mut().intern("Human");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "Whenever a player attacks" — closest expressible match is
                // the beginning of the declare-attackers step for any player.
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::DeclareAttackers,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: add_six_red,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}{R}{R}: Target creature gets +3/+0 until end of turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{R}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_target,
            }),
    )
}

fn add_six_red(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Until end of combat, you don't lose this mana as steps end" —
    // mana-retention rider is not expressible; only the addition is modeled.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source); 6],
    }]
}

fn pump_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 3,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
