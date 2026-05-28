//! Megatog — `{4}{R}{R}` 3/4 Creature — Atog.
//! Sacrifice an artifact: This creature gets +3/+3 and gains trample until end of turn.
//! GAP: ActivationCost::sacrifice is self-only; "sacrifice an artifact" (any) not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Megatog");
    let atog = reg.interner_mut().intern("Atog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(atog);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice an artifact: This creature gets +3/+3 and gains trample until end of turn.".into(),
                cost: ActivationCost { ..ActivationCost::default() },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_and_trample,
            }),
    )
}

fn pump_and_trample(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump { target: ctx.source, power: 3, toughness: 3, duration: Duration::EndOfTurn, keywords: vec![KeywordAbility::Trample] }]
}
