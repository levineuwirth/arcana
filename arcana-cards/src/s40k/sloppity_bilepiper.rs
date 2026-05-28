//! Sloppity Bilepiper — `{3}{B}` 3/3 black Demon.
//! "Jolly Gutpipes — {2}, {T}, Sacrifice a creature: The next creature spell you cast
//! this turn has cascade."
//! GAP: "the next creature spell you cast this turn has cascade" — granting cascade to
//! the next spell is a delayed continuous effect not in the catalog.
//! GAP: "Sacrifice a creature" (not self) — using sacrifice: true as approximation.

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
    let name = reg.interner_mut().intern("Sloppity Bilepiper");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}, Sacrifice a creature: The next creature spell you cast this turn has cascade.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").unwrap(),
                    tap: true,
                    // GAP: "Sacrifice a creature" (not self) — using sacrifice: true.
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_next_cascade,
            }),
    )
}

fn grant_next_cascade(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "the next creature spell you cast this turn has cascade" — delayed cascade
    // grant on next spell not in Effect catalog.
    Vec::new()
}
