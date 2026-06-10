//! Crawling Barrens — nonbasic land. "{T}: Add {C}." and "{4}: Put two
//! +1/+1 counters on this land. Then you may have it become a 0/0
//! Elemental creature until end of turn. It's still a land."
//!
//! The animation is modeled as counters + AddType(CREATURE) +
//! SetBasePT(0/0) until end of turn (additive — it stays a land). The
//! "you may" optionality and the Elemental subtype are documented gaps.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crawling Barrens");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}: Put two +1/+1 counters on this land. Then you \
                       may have it become a 0/0 Elemental creature until \
                       end of turn. It's still a land."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grow_and_animate,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn grow_and_animate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may have it become" — the optionality of the animation
    // is not modeled (animation always applies). The Elemental subtype
    // grant is also not expressible via AddType (types only).
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        },
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 0,
            toughness: 0,
            duration: Duration::EndOfTurn,
        },
    ]
}
