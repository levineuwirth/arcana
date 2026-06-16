//! Jeska, Thrice Reborn — `{2}{R}` legendary planeswalker, starting
//! loyalty 0 (she enters with a loyalty counter for each commander cast
//! from the command zone this game — a bespoke ETB-loyalty that the fixed
//! printed-loyalty field can't express; modeled as printed 0). Subtype
//! Jeska; mono-red. Partner.
//!
//! Loyalty abilities:
//! * `0`: Choose target creature. Until your next turn, its combat damage
//!   to your opponents is tripled. GAP — a combat-damage-tripling
//!   replacement effect is bespoke and not expressible.
//! * `−X`: Jeska deals X damage to each of up to three targets. OMITTED —
//!   `−X` (chosen) loyalty cost is not expressible (`remove_self_counter`
//!   is a fixed `u32`).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jeska, Thrice Reborn");
    let jeska = reg.interner_mut().intern("Jeska");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jeska);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(0),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "0: Choose target creature. Until your next turn, if that \
                   creature would deal combat damage to one of your \
                   opponents, it deals triple that damage to that player \
                   instead."
                .into(),
            cost: ActivationCost::default(),
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: true,
            activation_zone: arcana_core::registry::ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: zero_gap,
        }),
        // −X ability omitted: dynamic-X loyalty cost is not expressible.
    )
}

fn zero_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: combat-damage tripling replacement until your next turn is bespoke.
    Vec::new()
}
