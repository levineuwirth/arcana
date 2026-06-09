//! Groundling Pouncer — `{1}{G/U}` 2/1 green/blue Faerie.
//! "{G/U}: This creature gets +1/+3 and gains flying until end of turn.
//! Activate only once each turn and only if an opponent controls a creature with flying."
//! "only if an opponent controls a creature with flying" modeled via
//! `activation_condition` + `conditions::an_opponent_controls_a` (creature with Flying).
//! GAP: "Activate only once each turn" — per-turn activation limit not
//! expressible in ActivationCost.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::targets::ObjectFilter;
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
    let name = reg.interner_mut().intern("Groundling Pouncer");
    let faerie = reg.interner_mut().intern("Faerie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "Activate only once each turn" — per-turn activation
                // limit not expressible in ActivationCost.
                text: "{G/U}: This creature gets +1/+3 and gains flying until end of turn. Activate only once each turn and only if an opponent controls a creature with flying.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G/U}").unwrap(),
                    activation_condition: Some(|s, _src, you, _reg| {
                        arcana_core::conditions::an_opponent_controls_a(
                            s,
                            you,
                            &ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
                        )
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_and_fly,
            }),
    )
}

fn pump_and_fly(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Flying],
    }]
}
