//! Thundercloud Elemental — `{5}{U}{U}` 3/4 Elemental with Flying.
//! "{3}{U}: Tap all creatures with toughness 2 or less."
//! "{3}{U}: All other creatures lose flying until end of turn."
//!
//! Flying is a usable keyword. The first activation taps every creature
//! with toughness 2 or less (ForEach over the matching ids). The second
//! ("lose flying") has no keyword-removal effect — only LoseAllAbilities
//! exists, which would strip everything — so its effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thundercloud Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}: Tap all creatures with toughness 2 or less.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_small_creatures,
            })
            // GAP: "All other creatures lose flying until end of turn" — no
            // keyword-removal effect; LoseAllAbilities would strip everything.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}: All other creatures lose flying until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: lose_flying,
            }),
    )
}

fn tap_small_creatures(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().with_max_toughness(2),
        ctx.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Tap { target: NULL_OBJECT_ID }),
    }]
}

fn lose_flying(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no per-keyword removal effect.
    Vec::new()
}
