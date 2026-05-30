//! Fortune Teller's Talent — `{U}` blue Enchantment — Class.
//! Level 1 (base): You may look at the top card of your library any time.
//! {3}{U}: Level 2 — As long as you've cast a spell this turn, you may play
//!          cards from the top of your library.
//! {2}{U}: Level 3 — Spells you cast from anywhere other than your hand cost
//!          {2} less to cast.
//!
//! GAP: Level 1 "look at the top card of your library any time" — static
//!      continuous effect; not an anthem or keyword anthem (engine debt:
//!      continuous-effect engine subsystem).
//! GAP: Level 2 "as long as you've cast a spell this turn, you may play cards
//!      from the top of your library" — conditional play-from-top ability;
//!      not expressible (engine debt).
//! GAP: Level 3 "spells you cast from anywhere other than your hand cost {2}
//!      less" — cost-reduction continuous effect; not expressible (engine debt).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fortune Teller's Talent");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // {3}{U}: Level 2 (requires Level 1)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_2,
            })
            // {2}{U}: Level 3 (requires Level 2)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_3,
            }),
    )
}

fn level_up_to_2(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Level 2 per-level static "play cards from top of library when
    // you've cast a spell this turn" — deferred (continuous-effect engine).
    vec![Effect::AddCounters { target: ctx.source, kind: CounterKind::Level, count: 1 }]
}

fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Level 3 per-level static "spells cost {2} less from non-hand zones"
    // — deferred (continuous-effect engine).
    vec![Effect::AddCounters { target: ctx.source, kind: CounterKind::Level, count: 1 }]
}
