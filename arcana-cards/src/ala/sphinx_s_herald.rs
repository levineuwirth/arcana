//! Sphinx's Herald — `{U}` 1/1 Artifact Creature — Vedalken Wizard.
//! `{2}{U}, {T}, Sacrifice a white creature, a blue creature, and a black creature: Search
//! your library for a card named Sphinx Sovereign, put it onto the battlefield, then shuffle.`
//! GAP: "Sacrifice a white creature, a blue creature, and a black creature" — ActivationCost
//! sacrifice only sacrifices one creature; multi-sacrifice not expressible. GAP: TutorToBattlefield
//! by name not supported.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::Effect;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sphinx's Herald");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}, {T}, Sacrifice a white creature, a blue creature, and a black creature: Search your library for a card named Sphinx Sovereign, put it onto the battlefield, then shuffle.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}").unwrap(),
                    tap: true,
                    sacrifice: true,
                    // GAP: sacrifice only one creature; 3-color multi-sacrifice not expressible
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tutor_sphinx_sovereign,
            }),
    )
}

fn tutor_sphinx_sovereign(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: TutorToBattlefield by card name not supported — no registry-by-name lookup
    Vec::new()
}
