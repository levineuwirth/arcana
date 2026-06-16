//! Bane Alley Broker — `{1}{U}{B}` 0/3 Human Rogue.
//! "{T}: Draw a card, then exile a card from your hand face down."
//! "You may look at cards exiled with this creature." (static — info)
//! "{U}{B}, {T}: Return a card exiled with this creature to its
//! owner's hand."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bane Alley Broker");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    // GAP (static): "You may look at cards exiled with this creature." —
    // pure info-access static, no triggered/activated machinery.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Draw a card, then exile a card from your hand face down.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_then_exile,
            })
            // GAP: "{U}{B}, {T}: Return a card exiled with this creature to its
            // owner's hand." — no primitive can enumerate the cards this creature
            // exiled face-down (no per-source exile-link return effect). Bones-only.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}{B}, {T}: Return a card exiled with this creature to its owner's hand."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}{B}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: return_exiled_noop,
            }),
    )
}

fn draw_then_exile(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // "Draw a card" is expressible. GAP: "then exile a card from your hand
    // face down" — the face-down per-source exile link is not a primitive.
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}

fn return_exiled_noop(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
