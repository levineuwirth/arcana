//! Tuskeri Firewalker — `{2}{R}` 3/2 red Human Berserker. "Boast — {1}: Exile the
//! top card of your library. You may play that card this turn."
//! The effect is `Effect::ImpulseExile` (exile top card, may play it this turn).
//! The "attacked this turn" half of Boast is enforced via
//! `ActivationCost::activation_condition` + `conditions::source_attacked_this_turn`.
//! GAP: "only once each turn" — no per-turn activation counter on ActivationCost.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tuskeri Firewalker");
    let human = reg.interner_mut().intern("Human");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(berserker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: Exile the top card of your library. You may play that card this turn. (Boast — activate only if this creature attacked this turn, once each turn.)".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").unwrap(),
                    // Boast: only if this creature attacked this turn.
                    activation_condition: Some(|s, src, _you, _reg| {
                        arcana_core::conditions::source_attacked_this_turn(s, src)
                    }),
                    // GAP: "only once each turn" not expressible.
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: boast_exile_play,
            }),
    )
}

fn boast_exile_play(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Impulse: exile the top card; you may play it this turn.
    vec![Effect::ImpulseExile { player: ctx.controller, count: 1 }]
}
