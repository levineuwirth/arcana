//! Faith Healer — `{1}{W}` 1/1 Creature — Human Cleric.
//! Sacrifice an enchantment: You gain life equal to the sacrificed enchantment's mana value.
//! "Sacrifice an enchantment" cost modeled via `sacrifice_other` (enchantment
//! filter). GAP: "life equal to sacrificed enchantment's mana value" — can't
//! read the mana value of a just-sacrificed permanent (effect is a no-op).

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
    let name = reg.interner_mut().intern("Faith Healer");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice an enchantment: You gain life equal to the sacrificed enchantment's mana value.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(arcana_core::targets::ObjectFilter {
                        types: Some(TypeLine::ENCHANTMENT.into()),
                        ..Default::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_life_dynamic,
            }),
    )
}

fn gain_life_dynamic(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: amount equals sacrificed enchantment's mana value — no script helper for sacrificed permanent's CMC
    Vec::new()
}
