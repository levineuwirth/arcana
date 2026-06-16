//! Patron of the Moon — `{5}{U}{U}` 5/4 Legendary Creature — Spirit with
//! Flying. "{1}: Put up to two land cards from your hand onto the battlefield
//! tapped."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Patron of the Moon");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: "Moonfolk offering" is the Offering alternative-cast mechanic;
        // not in the usable keyword surface for this card class.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}: Put up to two land cards from your hand onto the battlefield tapped."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: put_two_lands,
        }),
    )
}

fn put_two_lands(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "up to two": each PutFromHandOntoBattlefield posts a (min-0) pick over
    // matching hand cards, so two sequential puts model "up to two".
    let land_filter = ObjectFilter::permanent().with_types(TypeLine::LAND.into());
    vec![Effect::Sequence(vec![
        Effect::PutFromHandOntoBattlefield {
            player: ctx.controller,
            filter: land_filter.clone(),
            tapped: true,
        },
        Effect::PutFromHandOntoBattlefield {
            player: ctx.controller,
            filter: land_filter,
            tapped: true,
        },
    ])]
}
