//! Ellywick Tumblestrum — `{2}{G}{G}` Legendary Planeswalker — Ellywick.
//! Starting loyalty inferred 4.
//! +1: Venture into the dungeon.
//! −2: Look at the top six cards of your library. You may reveal a creature
//!   card from among them and put it into your hand. If it's legendary, you
//!   gain 3 life. Put the rest on the bottom of your library in a random order.
//! −7: You get an emblem with "Creatures you control have trample and haste
//!   and get +2/+2 for each differently named dungeon you've completed."
//!
//! GAP: −2 "if it's legendary, you gain 3 life" — the conditional life-gain
//!   tied to the revealed card's legendary status is not expressible as a
//!   rider on DigTopN; the reveal-a-creature-to-hand portion is modeled.
//! GAP: −7 emblem grants a dynamic +2/+2-per-completed-dungeon anthem plus
//!   trample/haste; that emblem's static body is not expressible from the
//!   TriggeredAbilityDef-only EmblemDefinition surface. Declared, body GAP'd.

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ellywick Tumblestrum");
    let ellywick = reg.interner_mut().intern("Ellywick");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ellywick);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Venture into the dungeon.".into(),
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
                effect: plus_one_venture,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Look at the top six cards of your library. You may reveal a \
                       creature card from among them and put it into your hand. If it's \
                       legendary, you gain 3 life. Put the rest on the bottom of your \
                       library in a random order.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"Creatures you control have trample \
                       and haste and get +2/+2 for each differently named dungeon \
                       you've completed.\"".into(),
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
                effect: minus_seven_emblem,
            }),
    )
}

fn plus_one_venture(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Venture { player: ctx.controller }]
}

fn minus_two_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it's legendary, you gain 3 life" rider not expressible.
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 6,
        filter: Some(ObjectFilter::creature()),
        rest: DigRest::BottomRandom,
    }]
}

fn minus_seven_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with a dynamic per-completed-dungeon anthem + trample/haste
    // static; EmblemDefinition only carries triggered abilities, not statics.
    Vec::new()
}
