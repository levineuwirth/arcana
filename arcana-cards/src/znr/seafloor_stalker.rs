//! Seafloor Stalker — `{2}{U}` 2/3 Creature — Merfolk Rogue.
//! `{4}{U}: This creature gets +1/+0 until end of turn and can't be blocked this turn.
//!  This ability costs {1} less to activate for each creature in your party.`
//! NOTE: The "costs {1} less for each creature in your party" is a cost reduction GAP
//! (no cost-reduction mechanism in ActivationCost). Also "can't be blocked" is not in
//! the Effect catalog. Emitting best-effort: mana-only cost + Pump; the cost reduction
//! and unblockable rider are GAPs.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seafloor Stalker");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{U}: This creature gets +1/+0 until end of turn and can't be blocked this turn. This ability costs {1} less to activate for each creature in your party.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{U}").unwrap(),
                    ..ActivationCost::default()
                },
                // GAP: cost reduction per party creature not supported; "can't be blocked" not in Effect catalog
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
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
