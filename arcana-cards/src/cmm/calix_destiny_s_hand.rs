//! Calix, Destiny's Hand — `{2}{G}{W}` Legendary Planeswalker — Calix.
//! Starting loyalty inferred 5.
//! +1: Look at the top four cards of your library. You may reveal an
//!   enchantment card from among them and put that card into your hand. Put
//!   the rest on the bottom of your library in a random order.
//! −3: Exile target creature or enchantment you don't control until target
//!   enchantment you control leaves the battlefield.
//! −7: Return all enchantment cards from your graveyard to the battlefield.
//!
//! GAP: −3 is a linked-exile keyed to a SECOND target ("until target
//!   enchantment YOU control leaves the battlefield") — ExileUntilSourceLeaves
//!   ties the return to the SOURCE leaving, not an independent second target,
//!   so this two-target linked exile is not expressible. Declared with the
//!   correct −3 cost, effect GAP'd.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Calix, Destiny's Hand");
    let calix = reg.interner_mut().intern("Calix");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(calix);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top four cards of your library. You may reveal \
                       an enchantment card from among them and put that card into your \
                       hand. Put the rest on the bottom of your library in a random \
                       order.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Exile target creature or enchantment you don't control until \
                       target enchantment you control leaves the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_exile,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Return all enchantment cards from your graveyard to the \
                       battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_mass_return,
            }),
    )
}

fn plus_one_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 4,
        filter: Some(ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into())),
        rest: DigRest::BottomRandom,
    }]
}

fn minus_three_exile(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: linked exile keyed to a second independent target leaving the
    // battlefield is not expressible (ExileUntilSourceLeaves ties to source).
    Vec::new()
}

fn minus_seven_mass_return(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into());
    state
        .objects
        .objects_in_zone(Zone::Graveyard(ctx.controller))
        .filter(|o| filter.matches(o, state, ctx.controller))
        .map(|o| Effect::ReturnFromGraveyardToBattlefield { target: o.id })
        .collect()
}
