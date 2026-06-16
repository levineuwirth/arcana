//! Zariel, Archduke of Avernus — `{2}{R}{R}` Legendary Planeswalker — Zariel, starting loyalty 4.
//!
//! +1: Creatures you control get +1/+0 and gain haste until end of turn. The +1/+0
//!     anthem (until end of turn) is modeled via Effect::Anthem; the "gain haste"
//!     half of the grant is a controller-anchored filtered keyword grant with no
//!     demonstrated one-shot builder — GAP (pump modeled, haste omitted).
//! 0: Create a 1/1 red Devil creature token with "When this token dies, it deals 1
//!    damage to any target." The token is modeled; its embedded dies-triggered
//!    ability is not expressible on a TokenDefinition here — GAP (plain 1/1 Devil).
//! −6: You get an emblem with "At the end of the first combat phase on your turn,
//!     untap target creature you control. After this phase, there is an additional
//!     combat phase." The extra-combat-phase / first-combat-only timing is a rule-
//!     altering turn-structure effect the builders can't express — GAP (−6 shell
//!     preserved, emblem omitted).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zariel, Archduke of Avernus");
    let zariel = reg.interner_mut().intern("Zariel");
    let _devil = reg.interner_mut().intern("Devil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zariel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Creatures you control get +1/+0 and gain haste until end \
                       of turn.".into(),
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
                effect: plus_one_anthem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Create a 1/1 red Devil creature token with \"When this \
                       token dies, it deals 1 damage to any target.\"".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_devil,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"At the end of the first combat \
                       phase on your turn, untap target creature you control. After \
                       this phase, there is an additional combat phase.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

fn plus_one_anthem(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "and gain haste until end of turn" — a controller-anchored one-shot
    //      filtered keyword grant has no demonstrated builder. The +1/+0 anthem
    //      (until end of turn) is modeled.
    vec![Effect::Anthem {
        controller: ctx.controller,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
    }]
}

fn zero_devil(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let devil = reg.interner().lookup("Devil").expect("Devil interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(devil);
    // GAP: token's embedded "When this token dies, it deals 1 damage to any
    //      target" ability is not expressible on a TokenDefinition here — plain
    //      1/1 red Devil created.
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: devil,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn minus_six_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem "At the end of the first combat phase on your turn, untap target
    //      creature you control. After this phase, there is an additional combat
    //      phase." — first-combat-only timing plus an additional-combat-phase
    //      turn-structure alteration the demonstrated builders can't express.
    Vec::new()
}
