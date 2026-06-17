//! Tromell, Seymour's Butler — `{2}{G}` 2/3 Legendary Elf Advisor.
//! Each other nontoken creature you control enters with an additional
//! +1/+1 counter on it (static replacement — GAP).
//! `{1}, {T}: Proliferate X times`, X = nontoken creatures you control
//! that entered this turn.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tromell, Seymour's Butler");
    let elf = reg.interner_mut().intern("Elf");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static replacement "Each other nontoken creature you control
    // enters with an additional +1/+1 counter" — not a trigger/activated
    // ability and no enters-with replacement primitive in this surface.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Proliferate X times, where X is the number of nontoken creatures you control that entered this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: proliferate_x_times,
            }),
    )
}

fn proliferate_x_times(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X = "nontoken creatures you control that entered this turn"
    // is not computable from the allowed script:: helper surface, so the
    // repeat-count for Proliferate is unknown; emitting a fixed count
    // would be a materially wrong card.
    Vec::new()
}
