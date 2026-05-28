//! Retrofitted Transmogrant — `{B}` 1/1 Artifact Creature — Zombie.
//! `{3}{B}: Return this card from your graveyard to the battlefield tapped with two +1/+1 counters on it.`
//! GAP: ActivationZone::Graveyard not in catalog; using Battlefield placeholder.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Retrofitted Transmogrant");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}: Return this card from your graveyard to the battlefield tapped with two +1/+1 counters on it.".into(),
                cost: ActivationCost { mana_cost: ManaCost::parse("{3}{B}").unwrap(), exile_self: true, ..ActivationCost::default() },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                // GAP: ActivationZone::Graveyard not in catalog
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reanimate_self,
            }),
    )
}

fn reanimate_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: ctx.source },
        Effect::AddCounters { target: ctx.source, kind: CounterKind::PlusOnePlusOne, count: 2 },
    ]
}
