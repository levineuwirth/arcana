//! Veteran Warleader — `{1}{G}{W}` */* Human Soldier Ally.
//! Veteran Warleader's power and toughness are each equal to the number of
//! creatures you control.
//! Tap another untapped Ally you control: This creature gains your choice
//! of first strike, vigilance, or trample until end of turn.
//!
//! GAP: the characteristic-defining "power and toughness equal to the
//! number of creatures you control" is not expressible with the
//! demonstrated API; the base P/T is recorded as `*`/`*`.
//! GAP: "your choice of first strike, vigilance, or trample" is a modal
//! keyword grant — no modal keyword-choice API exists. The activated
//! ability grants FirstStrike as a best-effort stub.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Veteran Warleader");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    subtypes.0.insert(ally);

    let ally_filter = script::subtype_filter(reg, "Ally");

    // GAP: CDA "power and toughness equal to the number of creatures you
    // control" — recorded as */*.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Tap another untapped Ally you control: This creature gains \
                   your choice of first strike, vigilance, or trample until end \
                   of turn."
                .into(),
            cost: ActivationCost {
                tap_other: Some(ally_filter),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: grant_keyword_choice,
        }),
    )
}

fn grant_keyword_choice(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "your choice of first strike, vigilance, or trample" — no modal
    // keyword-choice API. Granting FirstStrike as a best-effort stub.
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::FirstStrike,
        duration: Duration::EndOfTurn,
    }]
}
