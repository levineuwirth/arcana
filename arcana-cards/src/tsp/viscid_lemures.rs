//! Viscid Lemures — `{4}{B}` 4/3 black Spirit.
//! "{0}: This creature gets -1/-0 and gains swampwalk until end of turn."
//!
//! GAP: Pump with negative power (-1/0) is expressible. Swampwalk is
//! Landwalk(Swamp) which requires interning "Swamp".
//! GAP: The -1/-0 pump at {0} cost is expressible but we need a free
//! activation cost. ActivationCost with zero mana is just
//! ActivationCost::default() (no mana, no tap).

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
    let name = reg.interner_mut().intern("Viscid Lemures");
    let spirit = reg.interner_mut().intern("Spirit");
    let swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{0}: This creature gets -1/-0 and gains swampwalk until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{0}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_swampwalk,
            }),
    )
}

fn pump_swampwalk(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let swamp = reg
        .interner()
        .lookup("Swamp")
        .expect("Swamp interned during register()");
    vec![
        Effect::Pump {
            target: ctx.source,
            power: -1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Landwalk(swamp),
            duration: Duration::EndOfTurn,
        },
    ]
}
