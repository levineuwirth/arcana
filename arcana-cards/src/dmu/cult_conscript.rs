//! Cult Conscript — `{B}` 2/1 Skeleton Warrior.
//! "This creature enters tapped." — GAP: no enters-tapped primitive in
//! the available surface.
//! "{1}{B}: Return this card from your graveyard to the battlefield.
//!  Activate only if a non-Skeleton creature died under your control this
//!  turn." — graveyard-activated; the reanimation is modeled by name.
//!
//! The activation condition is gated on `a_creature_died_this_turn`; the
//! exact "a non-Skeleton creature died under your control" refinement is a
//! fidelity GAP (broader predicate).

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cult Conscript");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}: Return this card from your graveyard to the battlefield. Activate only if a non-Skeleton creature died under your control this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    activation_condition: Some(if_creature_died),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_self,
            }),
    )
}

fn if_creature_died(
    state: &GameState,
    _source: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::a_creature_died_this_turn(state)
}

fn return_self(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let nm = reg.interner().lookup("Cult Conscript");
    vec![Effect::Reanimate {
        player: ctx.controller,
        filter: ObjectFilter { name: nm, ..ObjectFilter::default() },
        from_zone: Zone::Graveyard(ctx.controller),
    }]
}
