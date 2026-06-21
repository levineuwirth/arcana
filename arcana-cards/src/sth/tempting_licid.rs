//! Tempting Licid — `{2}{G}` 2/2 Licid.
//!
//! Oracle:
//! * {G}, {T}: This creature loses this ability and becomes an Aura
//!   enchantment with enchant creature. Attach it to target creature.
//!   You may pay {G} to end this effect. (GAP — the Licid
//!   become-an-Aura transform-and-attach is not an expressible Effect.
//!   The cost and target are wired; the body is GAP'd.)
//! * All creatures able to block enchanted creature do so. (GAP — a
//!   static "lure" on the enchanted creature; not a triggered/activated
//!   ability.)

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
    let name = reg.interner_mut().intern("Tempting Licid");
    let licid = reg.interner_mut().intern("Licid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(licid);

    // GAP: "All creatures able to block enchanted creature do so." — a
    // static lure on the enchanted creature; not expressible here.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{G}, {T}: This creature loses this ability and becomes an Aura enchantment with enchant creature. Attach it to target creature. You may pay {G} to end this effect.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: licid_transform,
        }),
    )
}

fn licid_transform(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Licid "becomes an Aura with enchant creature, attach to
    // target creature, pay {G} to end" transform is not expressible.
    Vec::new()
}
