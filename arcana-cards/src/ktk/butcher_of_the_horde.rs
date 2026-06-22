//! Butcher of the Horde — `{1}{R}{W}{B}` 5/4 Demon with Flying.
//!
//! * Flying (keyword).
//! * "Sacrifice another creature: This creature gains your choice of
//!   vigilance, lifelink, or haste until end of turn." — an activated
//!   ability with a sacrifice-another-creature cost. GAP: "your choice
//!   of vigilance, lifelink, or haste" is a modal keyword grant with no
//!   modal keyword-choice API; the ability grants Vigilance (first
//!   listed) as a best-effort stub (cf. Veteran Warleader).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Butcher of the Horde");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Sacrifice another creature: This creature gains your choice of vigilance, lifelink, or haste until end of turn.".into(),
            cost: ActivationCost {
                sacrifice_other: Some(ObjectFilter {
                    types: Some(TypeLine::CREATURE.into()),
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
            effect: gain_keyword,
        }),
    )
}

fn gain_keyword(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal "your choice of vigilance, lifelink, or haste" — no
    // keyword-choice API; grants Vigilance as a best-effort stub.
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Vigilance,
        duration: Duration::EndOfTurn,
    }]
}
