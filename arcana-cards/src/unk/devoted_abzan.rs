//! Devoted Abzan — `{B}` 1/1 black Dog Cleric. "{5}, {T}: You draw a card and lose
//! 1 life. This ability costs X less to activate, where X is your devotion to Abzan."
//! The devotion-based reduction (W/B/G mana symbols among the mana costs of
//! permanents you control) is wired via `ActivationCost::cost_reduction`
//! (`script::devotion`).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Devoted Abzan");
    let dog = reg.interner_mut().intern("Dog");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}, {T}: You draw a card and lose 1 life. (Costs X less where X = devotion to Abzan.)".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").unwrap(),
                    tap: true,
                    // "costs X less, where X is your devotion to Abzan
                    // (white/black/green)."
                    cost_reduction: Some(abzan_devotion_reduction),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_lose_life,
            }),
    )
}

/// "costs X less, where X is your devotion to Abzan" — devotion to
/// white, black, and green (CR 700.5) among permanents you control.
fn abzan_devotion_reduction(
    state: &GameState,
    _source: ObjectId,
    controller: PlayerId,
    _reg: &CardRegistry,
) -> u32 {
    let abzan = ColorSet::white() | ColorSet::black() | ColorSet::green();
    script::devotion(state, controller, abzan)
}

fn draw_lose_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::LoseLife { player: ctx.controller, amount: 1 },
    ]
}
