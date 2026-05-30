//! Ral, Monsoon Mage // Ral, Leyline Prodigy
//!
//! Front: {1}{R} Legendary Creature — Human Wizard 1/3
//! Instant and sorcery spells you cast cost {1} less to cast.
//! Whenever you cast an instant or sorcery spell during your turn, flip a coin.
//! If you lose the flip, Ral deals 1 damage to you.
//! If you win the flip, you may exile Ral. If you do, return him transformed.
//!
//! Back: Legendary Planeswalker — Ral (loyalty seeded on transform)
//! Ral enters with an additional loyalty counter for each instant/sorcery cast this turn.
//! +1: Until your next turn, instant and sorcery spells you cast cost {1} less to cast.
//! −2: Ral deals 2 damage divided as you choose among one or two targets. Draw a card if
//!     you control a blue permanent other than Ral.
//! −8: Exile the top eight cards. You may cast instant and sorcery spells from among them
//!     this turn without paying their mana costs.
//!
//! GAP: "cost {1} less to cast" static cost reduction — not expressible as an Effect,
//!      requires a layer-2 cost-reduction hook not yet in the engine.
//! GAP: back-face planeswalker activated abilities (+1, -2, -8) not modeled.
//! GAP: back-face ETB "additional loyalty counter per instant/sorcery cast this turn" not modeled.
//! GAP: "deals 2 divided as you choose among one or two targets" — split damage not expressible.
//! GAP: "exile top 8, cast for free" — exile-and-cast-from-exile not expressible.
//! GAP: back-face-only triggered/activated abilities not auto-installed on transform.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ral, Monsoon Mage");
    let sub_human = reg.interner_mut().intern("Human");
    let sub_wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_human);
    subtypes.0.insert(sub_wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ral, Leyline Prodigy");
    let sub_ral = reg.interner_mut().intern("Ral");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(sub_ral);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::blue(),
            types: TypeLine::PLANESWALKER.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            loyalty: Some(0), // seeded dynamically on transform by ETB counter count
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                    // Whenever you cast an instant or sorcery spell during your turn
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY))
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_instant_or_sorcery,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn on_instant_or_sorcery(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Flip a coin.
    // If you lose: Ral deals 1 damage to you.
    // If you win: you may exile Ral and return him transformed.
    // GAP: "you may exile Ral" choice is not expressible (OptionalPaymentKind has no
    //      free-exile-self cost); the exile-and-return step is approximated as
    //      Effect::Transform directly (no optional gate).
    vec![
        Effect::FlipCoin {
            player: trig.controller,
            win: Box::new(Effect::Transform { target: trig.source }),
            lose: Some(Box::new(Effect::DealDamage {
                target: DamageTarget::Player(trig.controller),
                amount: 1,
                source: trig.source,
            })),
        },
    ]
}
