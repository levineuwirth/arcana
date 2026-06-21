//! Calming Licid — `{2}{W}` 2/2 white Licid.
//!
//! {W}, {T}: This creature loses this ability and becomes an Aura
//! enchantment with enchant creature. Attach it to target creature.
//! You may pay {W} to end this effect.
//! Enchanted creature can't attack.
//!
//! The Licid mechanic (a creature turning itself into an attached Aura,
//! losing its activated ability, with a {W}-to-detach rider) has no
//! expressible shape in the demonstrated API. The activated ability is
//! registered with its real cost and target, but the
//! become-an-Aura-and-attach body is GAP'd, as is the conditional
//! static "enchanted creature can't attack."

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
    let name = reg.interner_mut().intern("Calming Licid");
    let licid = reg.interner_mut().intern("Licid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(licid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{W}, {T}: This creature loses this ability and becomes an Aura enchantment with enchant creature. Attach it to target creature. You may pay {W} to end this effect.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: licid_attach,
        }),
    )
    // GAP: static "Enchanted creature can't attack" — only relevant
    // once the Licid is attached as an Aura, which is itself unmodeled.
}

fn licid_attach(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Licid mechanic — creature becomes an Aura, loses this
    // ability, attaches to the target, with a {W}-to-detach rider. No
    // expressible shape.
    Vec::new()
}
