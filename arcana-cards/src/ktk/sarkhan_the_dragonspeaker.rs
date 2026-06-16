//! Sarkhan, the Dragonspeaker — `{3}{R}{R}` legendary planeswalker, starting loyalty 4.
//!
//! +1: Until end of turn, Sarkhan becomes a legendary 4/4 red Dragon creature
//!     with flying, indestructible, and haste.
//! −3: Sarkhan deals 4 damage to target creature.
//! −6: You get an emblem with two triggered abilities.
//!
//! Scope: the −3 direct-damage ability is fully expressed. The +1
//! "becomes a creature" animation (it must turn the planeswalker into a
//! creature with a printed P/T, color, type, and a bundle of keywords —
//! an integrated "becomes" transformation that the demonstrated Effect
//! surface can't assemble as one continuous self-animate) is GAP'd. The
//! −6 emblem is GAP'd (no loyalty-cost emblem builder demonstrated for
//! these multi-ability emblems with draw-step / end-step triggers).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sarkhan, the Dragonspeaker");
    let sarkhan = reg.interner_mut().intern("Sarkhan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sarkhan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
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
                text: "+1: Until end of turn, Sarkhan becomes a legendary 4/4 \
                       red Dragon creature with flying, indestructible, and \
                       haste.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_animate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Sarkhan deals 4 damage to target creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: You get an emblem with \"At the beginning of your \
                       draw step, draw two additional cards\" and \"At the \
                       beginning of your end step, discard your hand.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

/// `+1` — self-animate into a 4/4 Dragon. Not expressible as one
/// integrated "becomes a creature" continuous effect from the
/// demonstrated surface.
fn plus_one_animate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a 4/4 red Dragon creature with flying/indestructible/haste"
    // self-animation has no single demonstrated Effect.
    Vec::new()
}

/// `−3: Sarkhan deals 4 damage to target creature.`
fn minus_three_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }]
}

/// `−6` — emblem creation.
fn minus_six_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with draw-step/end-step triggered abilities.
    Vec::new()
}
