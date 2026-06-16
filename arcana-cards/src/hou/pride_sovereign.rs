//! Pride Sovereign — `{2}{G}` 2/2 Cat.
//!
//! This creature gets +1/+1 for each other Cat you control (static — see GAP).
//! {W}, {T}, Exert this creature: Create two 1/1 white Cat creature tokens
//! with lifelink.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pride Sovereign");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static "gets +1/+1 for each other Cat you control" is a continuous
    // self-buff, not a triggered/activated ability — not expressible here.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "Exert this creature" cost component is not expressible
                // (no exert cost field); only {W}, {T} are charged.
                text: "{W}, {T}, Exert this creature: Create two 1/1 white Cat creature tokens with lifelink.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_cats,
            }),
    )
}

fn make_cats(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let cat = reg.interner().lookup("Cat").unwrap_or_default();
    let make = || {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(cat);
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: cat,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Lifelink],
                abilities: vec![],
            },
        }
    };
    vec![make(), make()]
}
