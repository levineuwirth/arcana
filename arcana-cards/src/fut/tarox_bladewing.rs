//! Tarox Bladewing — `{2}{R}{R}{R}` 4/3 Legendary Dragon.
//!
//! "Flying, haste.
//!  Grandeur — Discard another card named Tarox Bladewing: Tarox Bladewing
//!  gets +X/+X until end of turn, where X is its power."
//!
//! Flying and haste are base keywords (Grandeur is not a separate keyword
//! variant — it is the activated ability below). The Grandeur ability pays
//! "discard another card named Tarox Bladewing" (modeled as `discard_other`
//! with a name filter) and pumps this creature by its own power.

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
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tarox Bladewing");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Grandeur — Discard another card named Tarox Bladewing: Tarox Bladewing gets +X/+X until end of turn, where X is its power.".into(),
                cost: ActivationCost {
                    discard_other: Some(ObjectFilter { name: Some(name), ..ObjectFilter::default() }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grandeur_pump,
            }),
    )
}

fn grandeur_pump(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::power_of(state, ctx.source).max(0);
    vec![Effect::Pump {
        target: ctx.source,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
