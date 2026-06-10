//! Whirling Catapult — `{4}` artifact (Alliances).
//! "{2}, Exile the top two cards of your library: This artifact deals
//! 1 damage to each creature with flying and each player." The
//! exile-top-two-library cost has no ActivationCost field and is a
//! GAP (only the {2} is charged); the each-creature-with-flying half
//! is also a GAP (no keyword filter), so only the each-player damage
//! is emitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Whirling Catapult");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{2}, Exile the top two cards of your library: This artifact deals 1 damage to each creature with flying and each player.".into(),
                // GAP: cost "Exile the top two cards of your library" — no
                // exile-from-library ActivationCost field; only the {2} mana
                // component is charged.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: damage_fliers_and_players,
            },
        ),
    )
}

fn damage_fliers_and_players(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "1 damage to each creature with flying" — ObjectFilter has no
    // keyword predicate in this API surface; only the each-player half is
    // emitted.
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: 1,
            source: ctx.source,
        })
        .collect()
}
