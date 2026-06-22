//! Urborg Phantom — `{2}{B}` 3/1 Creature — Spirit Minion.
//! "This creature can't block."
//! "{U}: Prevent all combat damage that would be dealt to and dealt by
//! this creature this turn."
//!
//! The "can't block" line is a static continuous ability and is GAP'd. The
//! activated ability prevents all damage dealt TO this creature; the "dealt
//! by this creature" half is a partial (no single-source prevention).

// GAP: "This creature can't block." — static combat-restriction
//      continuous ability, not expressible as a triggered/activated ability.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urborg Phantom");
    let spirit = reg.interner_mut().intern("Spirit");
    let minion = reg.interner_mut().intern("Minion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(minion);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}: Prevent all combat damage that would be dealt to \
                       and dealt by this creature this turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_damage_to_self,
            }),
    )
}

fn prevent_damage_to_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Prevents all damage dealt TO this creature this turn. The "and dealt
    // by this creature" half is a partial — no single-source prevention to
    // suppress damage this creature itself would deal.
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(ctx.source),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
