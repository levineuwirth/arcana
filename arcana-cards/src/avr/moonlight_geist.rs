//! Moonlight Geist — `{2}{W}` 2/1 Spirit with Flying.
//! "{3}{W}: Prevent all combat damage that would be dealt to and dealt
//! by this creature this turn."
//!
//! Flying is a base keyword. The activated ability prevents all damage
//! that would be dealt TO this creature this turn (best-effort
//! `PreventDamage` with `amount: None`). Fidelity gaps: the prevention
//! is not restricted to COMBAT damage, and the "dealt BY this creature"
//! half is not expressible (no per-source-object prevention).

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Moonlight Geist");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{W}: Prevent all combat damage that would be dealt to and dealt by this creature this turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: prevent_to_self,
        }),
    )
}

fn prevent_to_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Prevents all damage dealt TO this creature this turn. GAP: not
    // restricted to combat damage, and the "dealt by this creature"
    // half has no per-source-object prevention primitive.
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(ctx.source),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
