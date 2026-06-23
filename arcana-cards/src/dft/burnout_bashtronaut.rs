//! Burnout Bashtronaut — `{R}` 1/1 red Goblin Warrior with Menace.
//! "{2}: This creature gets +1/+0 until end of turn."
//! "Max speed — This creature has double strike." (static, speed-gated)
//!
//! Menace is a base keyword. "Start your engines!" and "Max speed" are the
//! Aetherdrift speed mechanic (not in the usable keyword surface), and the
//! "Max speed — double strike" line is a speed-gated static; both are
//! GAP'd. The `{2}` pump activation is wired faithfully.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Burnout Bashtronaut");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "Start your engines!" / "Max speed" — the speed mechanic is
        // not in the usable keyword surface.
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "{2}: This creature gets +1/+0 until end of turn."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: This creature gets +1/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            }),
        // GAP (static): "Max speed — This creature has double strike." — a
        // speed-gated continuous keyword grant, not a triggered/activated
        // ability.
    )
}

fn pump_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
