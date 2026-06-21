//! Psychatog — `{1}{U}{B}` 1/2 Atog.
//! "Discard a card: This creature gets +1/+1 until end of turn.
//!  Exile two cards from your graveyard: This creature gets +1/+1 until end
//!  of turn."
//!
//! Two activated pump abilities:
//! 1. "Discard a card: +1/+1" — cost is discarding a chosen card from hand
//!    (`discard_other`), payload pumps this creature.
//! 2. "Exile two cards from your graveyard: +1/+1" — there is no
//!    ActivationCost field for exiling cards from your graveyard as a cost,
//!    so the cost is GAP'd; the +1/+1 payload is recorded with no cost,
//!    which is not faithful, so the whole ability is GAP'd to avoid a
//!    cost-free pump.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Psychatog");
    let atog = reg.interner_mut().intern("Atog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(atog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Discard a card: This creature gets +1/+1 until end of turn.".into(),
            cost: ActivationCost {
                discard_other: Some(ObjectFilter::default()),
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
        // GAP: "Exile two cards from your graveyard: +1/+1" — there is no
        // ActivationCost field for exiling cards from your graveyard as a
        // cost; wiring the +1/+1 with no cost would be unfaithful, so this
        // second activated ability is omitted entirely.
    )
}

fn pump_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
