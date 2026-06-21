//! Crashing Centaur — `{4}{G}{G}` 3/4 Creature — Centaur.
//!
//! Oracle:
//! * {G}, Discard a card: This creature gains trample until end of turn.
//! * Threshold — As long as there are seven or more cards in your graveyard,
//!   this creature gets +2/+2 and has shroud. (Static conditional buff —
//!   GAP'd.)

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Crashing Centaur");
    let centaur = reg.interner_mut().intern("Centaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static — "Threshold — As long as there are seven or more cards in
    // your graveyard, this creature gets +2/+2 and has shroud" (continuous
    // conditional self-buff).
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{G}, Discard a card: This creature gains trample until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                discard_other: Some(ObjectFilter::default()),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: gain_trample,
        }),
    )
}

fn gain_trample(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Trample,
        duration: Duration::EndOfTurn,
    }]
}
