//! Hermitic Herbalist — `{G}{U}` 2/3 Human Druid Ally.
//! `{T}: Add one mana of any color.`
//! `{T}: Add two mana in any combination of colors. Spend this mana only to
//! cast Lesson spells.`

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
    let name = reg.interner_mut().intern("Hermitic Herbalist");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add one mana of any color.".into(),
                cost: ActivationCost { tap: true, ..ActivationCost::default() },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_any_color,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add two mana in any combination of colors. Spend this mana only to cast Lesson spells."
                    .into(),
                cost: ActivationCost { tap: true, ..ActivationCost::default() },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_any_color_lesson,
            }),
    )
}

fn add_any_color(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Add one mana of any color" — no choose-a-color mana primitive is
    // available; AddMana takes a fixed ManaColor per pip and emitting one
    // would be materially wrong.
    Vec::new()
}

fn add_two_any_color_lesson(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Add two mana in any combination of colors" with a Lesson-only
    // spend restriction — neither any-color choice nor the spend restriction
    // is expressible with the available mana primitives.
    Vec::new()
}
