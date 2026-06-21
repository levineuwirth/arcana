//! Cruel Deceiver — `{1}{B}` 2/1 Spirit.
//! "{1}: Look at the top card of your library.
//!  {2}: Reveal the top card of your library. If it's a land card,
//!  this creature gains 'Whenever this creature deals damage to a
//!  creature, destroy that creature' until end of turn. Activate only
//!  once each turn."
//!
//! Both activations are present as cost shapes. The first is a pure
//! information peek (no game-state change) with no demonstrated
//! primitive — GAP'd. The second's effect depends on inspecting the
//! revealed top card's type at resolution (not in the script helper
//! surface) and then conditionally granting a triggered ability — the
//! conditional reveal-and-grant is GAP'd; the `once_per_turn` limit is
//! captured on the cost.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cruel Deceiver");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
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
                text: "{1}: Look at the top card of your library.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: look_top,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: Reveal the top card of your library. If it's a land card, this creature gains \"Whenever this creature deals damage to a creature, destroy that creature\" until end of turn. Activate only once each turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reveal_and_grant,
            }),
    )
}

fn look_top(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Look at the top card of your library" — pure private-information peek; no game-state-changing effect primitive demonstrated.
    Vec::new()
}

fn reveal_and_grant(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal top card + (if land) grant a conditional triggered ability EOT — requires inspecting the revealed top card's type at resolution (not in script surface) before a conditional GrantTriggeredAbility.
    Vec::new()
}
