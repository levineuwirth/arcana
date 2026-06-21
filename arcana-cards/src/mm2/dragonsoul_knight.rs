//! Dragonsoul Knight — `{2}{R}` 2/2 Human Knight with First strike.
//!
//! Oracle:
//! * First strike — keyword line.
//! * {W}{U}{B}{R}{G}: Until end of turn, this creature becomes a
//!   Dragon, gets +5/+3, and gains flying and trample. — the +5/+3 and
//!   the flying/trample grants are expressed via one Pump; GAP: the
//!   "becomes a Dragon" subtype change has no add-subtype primitive
//!   (only `Effect::AddType` adds a card TYPE, not a creature subtype).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Dragonsoul Knight");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{W}{U}{B}{R}{G}: Until end of turn, this creature becomes a Dragon, gets +5/+3, and gains flying and trample.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: become_dragon,
        }),
    )
}

fn become_dragon(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a Dragon" (subtype change) — no add-subtype effect.
    vec![Effect::Pump {
        target: ctx.source,
        power: 5,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
    }]
}
