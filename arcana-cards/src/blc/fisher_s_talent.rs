//! Fisher's Talent — `{2}{G}{U}` green/blue Enchantment — Class.
//! Level 1: "At the beginning of your upkeep, look at top card. If it's a
//!           land, reveal it and create a 1/1 blue Fish token. Then draw."
//! Level 2: "If you would create a Fish token, create a 3/3 Shark instead."
//! Level 3: "If you would create a Shark token, create an 8/8 Octopus instead."
//! GAP: "look at top card and reveal if land" is too complex for Effect catalog.
//! GAP: Levels 2/3 replacement effects not expressible.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fisher's Talent");
    let class_sub = reg.interner_mut().intern("Class");
    let _fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // Level 1 trigger
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_look,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2 upgrade
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}{U}: Level 2".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}{U}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up,
            })
            // Level 3 upgrade
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}{U}: Level 3".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}{U}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up,
            }),
    )
}

fn upkeep_look(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top card, reveal if land, create Fish" conditional reveal
    // not expressible. Best-effort: draw a card (simplified)
    let fish = reg.interner().lookup("Fish").expect("Fish interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(fish);
    let token = TokenDefinition {
        name: fish,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        // GAP: should only create Fish if top card was a land
        Effect::CreateToken { controller: trig.controller, token },
        Effect::DrawCards { player: trig.controller, count: 1 },
    ]
}

fn level_up(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}
