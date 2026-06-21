//! Peregrin Took — `{2}{G}` 2/3 Legendary Halfling Citizen.
//!
//! "If one or more tokens would be created under your control, those tokens
//! plus an additional Food token are created instead." (token-creation
//! replacement — GAP'd)
//! "Sacrifice three Foods: Draw a card." (activated ability, faithful)
//!
//! Scryfall lists "Food" as a keyword tag; it is the Food token reference,
//! not a `KeywordAbility` variant, so the keyword line is empty.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

// GAP (replacement static): "If one or more tokens would be created under
// your control, those tokens plus an additional Food token are created
// instead" — a token-creation replacement effect with no documented Effect
// representation for this card class.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Peregrin Took");
    let halfling = reg.interner_mut().intern("Halfling");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(citizen);

    // Ensure "Food" is interned so the sacrifice filter resolves it.
    let _food = reg.interner_mut().intern("Food");
    let food_filter = script::subtype_filter(reg, "Food");

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

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Sacrifice three Foods: Draw a card.".into(),
            cost: ActivationCost {
                sacrifice_other: Some(food_filter),
                sacrifice_other_count: 3,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: draw_a_card,
        }),
    )
}

fn draw_a_card(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
