//! Stormcloud Djinn — `{4}{U}` 3/3 Djinn with Flying.
//! "This creature can block only creatures with flying.
//!  {R}{R}: This creature gets +2/+0 until end of turn and deals 1
//!  damage to you."
//!
//! Flying is the keyword line. The "can block only creatures with
//! flying" line is a static blocking restriction with no expressible
//! primitive (GAP'd). The activated ability pumps the source +2/+0
//! until end of turn and makes its controller lose 1 life.

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
    let name = reg.interner_mut().intern("Stormcloud Djinn");
    let djinn = reg.interner_mut().intern("Djinn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "This creature can block only creatures with flying" —
    //      a blocking restriction with no expressible primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}{R}: Stormcloud Djinn gets +2/+0 until end of turn \
                       and deals 1 damage to you."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_and_self_damage,
            }),
    )
}

fn pump_and_self_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Pump {
            target: ctx.source,
            power: 2,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::LoseLife {
            player: ctx.controller,
            amount: 1,
        },
    ]
}
