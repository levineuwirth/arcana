//! Tempest Efreet — `{1}{R}{R}{R}` 3/3 Efreet.
//! "{T}, Sacrifice this creature: Target opponent may pay 10 life. If that
//!  player doesn't, they reveal a card at random from their hand. Exchange
//!  ownership of the revealed card and Tempest Efreet. …This change in
//!  ownership is permanent."
//!
//! The ante / ownership-exchange payload is not expressible with the
//! engine's effect API (no random hand reveal, no ownership transfer
//! between players), so the cost is modeled faithfully ({T} + sacrifice
//! self) and the effect body is GAP'd. The "Remove this card from your
//! deck before playing if you're not playing for ante" line is format
//! text, not an ability.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tempest Efreet");
    let efreet = reg.interner_mut().intern("Efreet");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(efreet);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice this creature: Target opponent may pay 10 life. If that player doesn't, they reveal a card at random from their hand and exchange its ownership with Tempest Efreet permanently.".into(),
            cost: ActivationCost {
                tap: true,
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_opponent()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: ante_exchange,
        }),
    )
}

fn ante_exchange(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: random hand reveal + permanent ownership exchange (ante
    // mechanic) — not expressible with the engine's effect API.
    Vec::new()
}
