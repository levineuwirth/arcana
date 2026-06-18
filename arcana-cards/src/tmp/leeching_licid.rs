//! Leeching Licid — `{1}{B}` 1/1 Licid.
//! "{B}, {T}: This creature loses this ability and becomes an Aura
//! enchantment with enchant creature. Attach it to target creature.
//! You may pay {B} to end this effect."
//! "At the beginning of the upkeep of enchanted creature's controller,
//! this creature deals 1 damage to that player."
//!
//! The Licid type-changing self-attach loop is not expressible (no
//! Effect makes a creature lose its ability and become an Aura with a
//! continuing detach option). The upkeep trigger keys off "enchanted
//! creature's controller", which only exists in the Aura state, so it
//! too is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leeching Licid");
    let licid = reg.interner_mut().intern("Licid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(licid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, {T}: This creature loses this ability and becomes an Aura enchantment with enchant creature. Attach it to target creature. You may pay {B} to end this effect.".into(),
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
                effect: licid_becomes_aura,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: upkeep_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn licid_becomes_aura(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Licid loop — "loses this ability and becomes an Aura enchantment
    // with enchant creature, attach it, you may pay {B} to end this effect"
    // has no faithful Effect (no in-place creature→Aura conversion).
    Vec::new()
}

fn upkeep_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: keys off "enchanted creature's controller", which only exists
    // once this card has become an Aura (the Licid state above is unmodeled).
    Vec::new()
}
