//! Ranger Class — {1}{G} green Enchantment — Class.
//! Level 1: "When this Class enters, create a 2/2 green Wolf creature token."
//! Level 2 ({1}{G}): "Whenever you attack, put a +1/+1 counter on target
//!                    attacking creature."
//! Level 3 ({3}{G}): "You may look at the top card of your library any time.
//!                    You may cast creature spells from the top of your library."
//!
//! The level-1 ETB token is installed from a SelfEntersBattlefield trigger
//! (the base ability is on as soon as the Class enters).
//!
//! GAP: Level 2's granted triggered ability ("Whenever you attack, put a
//!      +1/+1 counter on target attacking creature") is not a P/T or keyword
//!      anthem, so it is per-level granted-ability continuous-effect engine
//!      debt — the install-on-level-up builders only cover anthems. The
//!      level-up cost and Level-2 precondition are modeled faithfully.
//! GAP: Level 3's statics ("look at the top card of your library any time",
//!      "cast creature spells from the top of your library") are per-level
//!      granted abilities deferred (continuous-effect engine subsystem).

use arcana_core::effects::{Effect, TokenDefinition};
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
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ranger Class");
    let class_sub = reg.interner_mut().intern("Class");
    let _ = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // Level 1 (base): ETB create a 2/2 green Wolf.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_wolf,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2: {1}{G}
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G}: Level 2".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 1)),
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
            // Level 3: {3}{G}
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}: Level 3".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}").unwrap(),
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

fn make_wolf(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let wolf_sym = reg.interner().lookup("Wolf");
    let mut token_subtypes = SubtypeSet::default();
    if let Some(s) = wolf_sym {
        token_subtypes.0.insert(s);
    }
    let token_name = reg.interner().lookup("Wolf").unwrap_or_default();
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn level_up(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}
