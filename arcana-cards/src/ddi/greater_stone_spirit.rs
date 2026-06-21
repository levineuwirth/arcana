//! Greater Stone Spirit — `{4}{R}{R}` 4/4 Creature — Elemental Spirit.
//!
//! Oracle text:
//! * "This creature can't be blocked by creatures with flying." — GAP: a
//!   static, source-filtered can't-be-blocked-by evasion is not expressible
//!   (the available `CantBeBlocked` is an unconditional full block, not a
//!   "by [filter]" restriction).
//! * "{2}{R}: Until end of turn, target creature gets +0/+2 and gains
//!   '{R}: This creature gets +1/+0 until end of turn.'" — the +0/+2 pump is
//!   modeled. GAP: granting an arbitrary ACTIVATED ability is not expressible
//!   (only triggered-ability grants exist), so the granted "{R}: +1/+0"
//!   sub-ability is omitted.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Greater Stone Spirit");
    let elemental = reg.interner_mut().intern("Elemental");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "can't be blocked by creatures with flying" — filtered static
    // evasion not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}: Until end of turn, target creature gets +0/+2 and gains \"{R}: This creature gets +1/+0 until end of turn.\"".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_toughness,
            }),
    )
}

fn pump_toughness(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: the granted "{R}: +1/+0" activated ability is omitted; +0/+2 only.
    vec![Effect::Pump {
        target: *id,
        power: 0,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
