//! Lobelia, Defender of Bag End — `{2}{B}` 2/2 Legendary Creature — Halfling
//! Citizen.
//!
//! Oracle:
//! * When Lobelia enters, look at the top card of each opponent's library and
//!   exile those cards face down. (GAP)
//! * {T}, Sacrifice an artifact: Choose one —
//!   • Until end of turn, you may play a card exiled with Lobelia without paying
//!     its mana cost. (GAP)
//!   • Each opponent loses 2 life and you gain 2 life.
//!
//! GAP: the ETB "exile the top card of each opponent's library face down" has
//! no primitive for face-down exile of another player's library card with later
//! play permission.
//! GAP: the activated ability is modal ("choose one"), which is not expressible
//! on an activated ability; mode 1 ("play a card exiled with Lobelia for free")
//! depends on the GAP'd ETB exile. We wire the fully-expressible mode 2 (each
//! opponent loses 2 life, you gain 2 life) unconditionally.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lobelia, Defender of Bag End");
    let halfling = reg.interner_mut().intern("Halfling");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_opponents_tops,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice an artifact: Each opponent loses 2 life and you gain 2 life.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: drain_two,
            }),
    )
}

fn etb_exile_opponents_tops(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no primitive exiles the top card of another player's library face
    // down with later play permission keyed to this source.
    Vec::new()
}

fn drain_two(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, ctx.controller)
        .into_iter()
        .map(|opp| Effect::LoseLife { player: opp, amount: 2 })
        .collect();
    effects.push(Effect::GainLife {
        player: ctx.controller,
        amount: 2,
    });
    vec![Effect::Sequence(effects)]
}
