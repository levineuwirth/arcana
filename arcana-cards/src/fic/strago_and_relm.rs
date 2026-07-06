//! Strago and Relm — `{2}{R}` 1/3 red Legendary Human Wizard.
//! "Sketch and Lore — {2}{R}, {T}: Target opponent exiles cards from the top of their
//! library until they exile an instant, sorcery, or creature card. You may cast that
//! card without paying its mana cost. If you cast a creature spell this way, it gains
//! haste and 'At the beginning of the end step, sacrifice this creature.'"
//! GAP: This is a complex multi-step Imprint-style effect not expressible in catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Strago and Relm");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}, {T}: Target opponent exiles cards until instant/sorcery/creature; you may cast it free.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_opponent()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: sketch_and_lore,
            }),
    )
}

fn sketch_and_lore(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target opponent exiles cards until finding instant/sorcery/creature,
    // you may cast it free with conditional haste+sac" — complex Imprint/Cascade-
    // like multi-step effect not in catalog.
    Vec::new()
}
