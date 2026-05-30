//! Voltaic Visionary // Volt-Charged Berserker — `{1}{R}` Human Wizard 3/1 (front).
//! Front face:
//!   {T}: This creature deals 2 damage to you. Exile the top card of your library.
//!        You may play that card this turn. Activate only as a sorcery.
//!   When you play a card exiled with this creature, transform this creature.
//! Back face (Volt-Charged Berserker):
//!   This creature can't block.
//!
//! GAP: Activated ability tap effect "exile top card, may play this turn" — impulse-play
//!      tracking (exile-specific play-this-turn flag) not in engine; ability GAP'd.
//! GAP: Transform trigger "when you play a card exiled with this creature" — exile-source
//!      tracking not expressible; trigger GAP'd.
//! GAP: Back-face-only static "this creature can't block" not auto-installed on transform.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationZone, CardDefinition, CardFace, CardRegistry,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voltaic Visionary");
    let human_sub = reg.interner_mut().intern("Human");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Volt-Charged Berserker — Human Berserker
    let back_name = reg.interner_mut().intern("Volt-Charged Berserker");
    let back_human_sub = reg.interner_mut().intern("Human");
    let back_berserker_sub = reg.interner_mut().intern("Berserker");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_human_sub);
    back_subtypes.0.insert(back_berserker_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {T}: deals 2 damage to you, exile top card, may play this turn — sorcery speed.
            // GAP: exile-top + play-this-turn impulse not expressible; only the self-damage fires.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: This creature deals 2 damage to you. Exile the top card of your library. You may play that card this turn. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: visionary_tap,
            }),
        // GAP: "When you play a card exiled with this creature, transform" trigger not modeled
        //      (exile-source tracking not in engine).
        // GAP: back-face static "this creature can't block" not auto-installed on transform.
    )
}

fn visionary_tap(
    _state: &arcana_core::state::GameState,
    ctx: &arcana_core::registry::ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Self-damage to controller; impulse-play is GAP.
    vec![Effect::DealDamage {
        target: DamageTarget::Player(ctx.controller),
        amount: 2,
        source: ctx.source,
    }]
}
