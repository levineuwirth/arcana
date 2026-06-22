//! Sarcatog — `{1}{B}{R}` 1/2 Atog.
//!
//! * Exile two cards from your graveyard: This creature gets +1/+1 until end
//!   of turn.  (GAP — no exile-from-graveyard activation cost field.)
//! * Sacrifice an artifact: This creature gets +1/+1 until end of turn.
//!
//! The graveyard-exile cost has no `ActivationCost` field (the cost catalog
//! has no "exile N cards from your graveyard" cost), so that whole ability is
//! GAP'd. The sacrifice-an-artifact ability maps to `sacrifice_other`.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sarcatog");
    let atog = reg.interner_mut().intern("Atog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(atog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Exile two cards from your graveyard: this creature gets +1/+1
    //       until end of turn." — no exile-from-graveyard cost field exists
    //       in ActivationCost; the whole ability is unexpressible.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice an artifact: This creature gets +1/+1 until end of turn.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::ARTIFACT.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
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
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
