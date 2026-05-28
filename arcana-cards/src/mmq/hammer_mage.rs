//! Hammer Mage — `{1}{R}` 1/1 Human Spellshaper.
//! `{X}{R}, {T}, Discard a card: Destroy all artifacts with mana value X or less.`
//! GAP: ActivationCost has no "discard a card" field; also dynamic X-based CMC filter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hammer Mage");
    let human = reg.interner_mut().intern("Human");
    let spellshaper = reg.interner_mut().intern("Spellshaper");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(spellshaper);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{R}, {T}, Discard a card: Destroy all artifacts with mana value X or less.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_small_artifacts,
            }),
    )
}

fn destroy_small_artifacts(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: ActivationCost has no "discard a card" field.
    // Using x_value to determine CMC threshold; if no X, default to 0.
    let x = ctx.x_value.unwrap_or(0);
    let ids = script::ids_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into())
            .with_max_cmc(x),
        ctx.controller,
    );
    ids.into_iter()
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect()
}
