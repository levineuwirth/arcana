//! Corrupting Licid — `{2}{B}` 2/2 Licid.
//!
//! * `{B}, {T}: This creature loses this ability and becomes an Aura
//!   enchantment with enchant creature. Attach it to target creature. You
//!   may pay {B} to end this effect.` — partially modeled: the activation
//!   targets a creature and attaches this permanent to it via `Effect::Attach`.
//!   The Licid transform ("loses this ability and becomes an Aura") and the
//!   "{B} to end this effect" detach are GAP'd (no such effects).
//! * Enchanted creature has fear. — GAP'd: a static granted to the enchanted
//!   creature is not a triggered/activated ability and depends on the Licid
//!   transform that isn't modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Corrupting Licid");
    let licid = reg.interner_mut().intern("Licid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(licid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static — "Enchanted creature has fear." Depends on the Licid
    // becomes-an-Aura transform, which is not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, {T}: This creature loses this ability and becomes an Aura \
                       enchantment with enchant creature. Attach it to target creature. \
                       You may pay {B} to end this effect."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
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
}

fn licid_attach(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // Best-effort: attach this permanent to the chosen creature. The
    // "loses this ability and becomes an Aura" transform and the optional
    // "{B} to end this effect" detach are GAP'd (no such effects).
    vec![Effect::Attach {
        equipment_or_aura: ctx.source,
        target: *id,
    }]
}
