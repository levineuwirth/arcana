//! Atreus, Impulsive Son — `{1}{U}{R}` 2/4 Legendary God Archer (U/R).
//! Reach.
//! {3}, {T}: Draw a card for each experience counter you have, then
//! discard a card. Atreus deals 2 damage to each opponent.
//! Partner—Father & son.
//!
//! Reach is emitted. "Partner" is not a KeywordAbility variant — GAP'd.
//! The activated ability emits the discard and the 2-damage-to-each-
//! opponent payload; the "draw a card for each experience counter you
//! have" portion is GAP'd (player experience counters are not readable
//! through the available script helpers).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Atreus, Impulsive Son");
    let god = reg.interner_mut().intern("God");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP: "Partner—Father & son." — Partner is not a KeywordAbility variant.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}, {T}: Draw a card for each experience counter you have, then discard a card. Atreus deals 2 damage to each opponent.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: atreus_ability,
            }),
    )
}

fn atreus_ability(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Draw a card for each experience counter you have" — player
    //      experience counters are not readable through script helpers.
    let mut effects = vec![Effect::Discard {
        player: ctx.controller,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }];
    for opp in script::opponents(state, ctx.controller) {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(opp),
            amount: 2,
        });
    }
    effects
}
