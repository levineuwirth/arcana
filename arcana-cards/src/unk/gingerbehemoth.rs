//! Gingerbehemoth — `{6}` 6/6 Artifact Creature — Food Golem with
//! Vigilance and Trample.
//! "Gingerbehemoth costs {2} less to cast for each Food you've
//!  sacrificed this turn." (static cost reduction — GAP'd)
//! "{4}, {T}, Sacrifice Gingerbehemoth: You gain 6 life."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gingerbehemoth");
    let food = reg.interner_mut().intern("Food");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(food);
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "costs {2} less to cast for each Food you've sacrificed this turn" — a static cost-reduction effect.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{4}, {T}, Sacrifice Gingerbehemoth: You gain 6 life.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                tap: true,
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: gain_six_life,
        }),
    )
}

fn gain_six_life(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife { player: ctx.controller, amount: 6 }]
}
