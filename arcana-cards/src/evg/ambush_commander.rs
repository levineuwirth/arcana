//! Ambush Commander — `{3}{G}{G}` 2/2 Elf.
//!
//! Forests you control are 1/1 green Elf creatures that are still lands.
//! {1}{G}, Sacrifice an Elf: Target creature gets +3/+3 until end of turn.
//!
//! The activated ability is wired: pay {1}{G} and sacrifice an Elf you
//! control (via the `sacrifice_other` cost field), pumping target creature
//! +3/+3 until end of turn. GAP: "Forests you control are 1/1 green Elf
//! creatures that are still lands" is a continuous land-animation static
//! with no demonstrated registration-time primitive.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ambush Commander");
    let elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    let sac_elf = script::subtype_filter(reg, "Elf");
    // GAP: "Forests you control are 1/1 green Elf creatures that are still
    // lands" — continuous land-animation static, no primitive.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{G}, Sacrifice an Elf: Target creature gets +3/+3 until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                sacrifice_other: Some(sac_elf),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_target,
        }),
    )
}

fn pump_target(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 3,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
