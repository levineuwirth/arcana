//! Inferno of the Star Mounts — `{4}{R}{R}` Legendary 6/6 Creature — Dragon.
//! "This spell can't be countered."
//! "Flying, haste"
//! "{R}: Inferno of the Star Mounts gets +1/+0 until end of turn. When its
//!  power becomes 20 this way, it deals 20 damage to any target."
//!
//! Decomposition:
//! - Keyword line: Flying, Haste.
//! - GAP: "This spell can't be countered." is a static cast-time ability — no
//!   expressible Effect.
//! - Activated `{R}: …` → +1/+0 until end of turn on this creature
//!   (expressible). GAP: the "when its power becomes 20 this way, it deals 20
//!   damage to any target" power-threshold rider is not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inferno of the Star Mounts");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{R}: Inferno of the Star Mounts gets +1/+0 until end of turn. When its power \
                   becomes 20 this way, it deals 20 damage to any target."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_self,
        }),
    )
}

/// +1/+0 until end of turn on this creature. GAP: the "when its power becomes
/// 20 this way, it deals 20 damage to any target" rider is not expressible.
fn pump_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
