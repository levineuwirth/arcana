//! Atogatog — `{W}{U}{B}{R}{G}` 5/5 Legendary Creature — Atog.
//! `Sacrifice an Atog creature: Atogatog gets +X/+X until end of turn, where X is the
//!  sacrificed creature's power.`
//! NOTE: "Sacrifice an Atog creature" — ActivationCost.sacrifice = true sacrifices SELF.
//! Sacrificing another creature is a GAP. Also X = sacrificed creature's power not trackable.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Atogatog");
    let atog = reg.interner_mut().intern("Atog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(atog);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice an Atog creature: Atogatog gets +X/+X until end of turn, where X is the sacrificed creature's power.".into(),
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                // GAP: sacrifice = true sacrifices SELF, not another Atog; X = sacrificed power not trackable
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            }),
    )
}

fn pump_self(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X = sacrificed creature's power — not accessible after sacrifice is paid
    Vec::new()
}
