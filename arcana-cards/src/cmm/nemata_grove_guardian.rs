//! Nemata, Grove Guardian — `{4}{G}{G}` 4/5 Legendary Creature — Treefolk (green).
//!
//! Two activated abilities:
//!   1. `{2}{G}: Create a 1/1 green Saproling creature token.`
//!   2. `Sacrifice a Saproling: Saproling creatures get +1/+1 until end of turn.`
//!
//! Ability 1 mints a hand-rolled 1/1 green Saproling token. Ability 2 takes a
//! CHOSEN Saproling as its cost (`sacrifice_other`, not the source) and pumps
//! every Saproling creature on the battlefield (board-wide, per oracle) +1/+1
//! until end of turn.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nemata, Grove Guardian");
    let treefolk = reg.interner_mut().intern("Treefolk");
    // Pre-intern the token subtype so it exists at resolve time.
    let _saproling = reg.interner_mut().intern("Saproling");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}: Create a 1/1 green Saproling creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_saproling,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice a Saproling: Saproling creatures get +1/+1 until end of turn."
                    .into(),
                cost: ActivationCost {
                    sacrifice_other: Some(script::subtype_filter(reg, "Saproling")),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_saprolings,
            }),
    )
}

/// `{2}{G}`: create one 1/1 green Saproling creature token.
fn make_saproling(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let name = reg.interner().lookup("Saproling").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(sym) = reg.interner().lookup("Saproling") {
        subtypes.0.insert(sym);
    }
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// Sacrifice a Saproling: every Saproling creature gets +1/+1 until end of turn.
fn pump_saprolings(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Saproling");
    let ids = script::ids_matching(state, &filter, ctx.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: arcana_core::objects::NULL_OBJECT_ID,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
