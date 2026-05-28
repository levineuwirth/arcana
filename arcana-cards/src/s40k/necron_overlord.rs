//! Necron Overlord — `{2}{B}{B}` 2/5 Artifact Creature — Necron Noble.
//! Relentless March — `{X}, {T}, Tap X untapped artifacts you control: Target opponent loses X life.`
//! GAP: "{X}" variable cost with "tap X untapped artifacts" as cost — no ActivationCost field
//! for tapping other permanents. X-cost mana is also not directly expressible. Emitting
//! placeholder with target player; effect amount is a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Necron Overlord");
    let necron = reg.interner_mut().intern("Necron");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(necron);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Relentless March — {X}, {T}, Tap X untapped artifacts you control: Target opponent loses X life.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                // GAP: {X} variable cost and "tap X untapped artifacts" not in ActivationCost
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: relentless_march,
            }),
    )
}

fn relentless_march(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X value from tapped artifacts not computable; variable amount dynamic X not supported
    Vec::new()
}
