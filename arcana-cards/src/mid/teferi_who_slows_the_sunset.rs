//! Teferi, Who Slows the Sunset — `{2}{W}{U}` Legendary Planeswalker — Teferi, loyalty 5.
//!
//! +1: Choose up to one target artifact, up to one target creature, and up to one
//!     target land. Untap the chosen permanents you control; tap the chosen ones you
//!     don't control; you gain 2 life. The three-typed up-to-one-each targeting with
//!     a controller-conditional tap/untap split is not expressible from the
//!     demonstrated surface; the GainLife half is modeled, the rest GAP'd.
//! −2: Look at the top three cards of your library. Put one of them into your hand
//!     and the rest on the bottom in any order. Look-and-select is not expressible —
//!     GAP body, shell kept with correct cost.
//! −7: You get an emblem with "Untap all permanents you control during each
//!     opponent's untap step" and "You draw a card during each opponent's draw step."
//!     Both clauses alter opponents' steps (not your-step StepBegins triggers) and are
//!     not expressible by anthem/keyword/standard StepBegins-You triggers. Emblem
//!     shell created (interned name, empty statics/abilities) with the grant GAP'd.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teferi, Who Slows the Sunset");
    let teferi = reg.interner_mut().intern("Teferi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(teferi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Choose up to one target artifact, up to one target \
                       creature, and up to one target land. Untap the chosen \
                       permanents you control. Tap the chosen permanents you don't \
                       control. You gain 2 life."
                    .into(),
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
                effect: plus_one_gain_life,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Look at the top three cards of your library. Put one of \
                       them into your hand and the rest on the bottom of your \
                       library in any order."
                    .into(),
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
                text: "-7: You get an emblem with \"Untap all permanents you control \
                       during each opponent's untap step\" and \"You draw a card \
                       during each opponent's draw step.\""
                    .into(),
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

fn plus_one_gain_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the up-to-one-each (artifact/creature/land) targeting with a
    //      controller-conditional untap-yours / tap-theirs split is not expressible
    //      from the demonstrated surface. The "you gain 2 life" half is modeled.
    vec![Effect::GainLife { player: ctx.controller, amount: 2 }]
}

fn minus_two_dig(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top three, put one in hand, rest on bottom" — look-and-select is
    //      not expressible from the demonstrated Effect surface.
    Vec::new()
}

fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Teferi, Who Slows the Sunset").expect("name interned");
    // GAP: both emblem clauses alter opponents' steps ("during each opponent's untap
    //      step" / "draw step"); they are not expressible by anthem/keyword or a
    //      standard StepBegins-You triggered ability. Emblem shell created.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
