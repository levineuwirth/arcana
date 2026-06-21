//! The Grand Goatnapper — `{2}{B}{R}` 3/4 Legendary Creature — Goblin Rogue.
//!
//! Oracle:
//! * Spells you cast have affinity for Goats. (Static cost-reduction — GAP'd.)
//! * {T}: Another target non-Goat creature perpetually becomes a Goat in
//!   addition to its other types. Then conjure a card named Goatnap into your
//!   hand. Activate only as a sorcery.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Grand Goatnapper");
    let goblin = reg.interner_mut().intern("Goblin");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static — "Spells you cast have affinity for Goats" (cost-reduction
    // granting static; not expressible).
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Another target non-Goat creature perpetually becomes a Goat in addition to its other types. Then conjure a card named Goatnap into your hand. Activate only as a sorcery."
                .into(),
            cost: ActivationCost::tap_only(),
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: goatnap,
        }),
    )
}

fn goatnap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "perpetually becomes a Goat in addition to its other types" — no
    // AddSubtype effect and no perpetual/permanent subtype-grant duration;
    // AddType adds card TYPES, not creature subtypes.
    // GAP: Conjure not modeled (Arena-only mechanic; "conjure a card named
    // Goatnap into your hand" would need registry-by-name creation).
    Vec::new()
}
