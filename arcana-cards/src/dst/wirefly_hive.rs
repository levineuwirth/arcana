//! Wirefly Hive — `{3}` artifact.
//! "{3}, {T}: Flip a coin. If you win the flip, create a 2/2 colorless
//! Insect artifact creature token with flying named Wirefly. If you
//! lose the flip, destroy all permanents named Wirefly."
//!
//! `Effect::FlipCoin` carries both branches: the win branch mints the
//! Wirefly token, the lose branch sweeps every permanent named Wirefly
//! via `ForEach` over a name-filtered id enumeration.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wirefly Hive");
    // Pre-intern the token name and subtype for resolve-time lookup.
    let _wirefly = reg.interner_mut().intern("Wirefly");
    let _insect = reg.interner_mut().intern("Insect");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}, {T}: Flip a coin. If you win the flip, create a 2/2 colorless Insect artifact creature token with flying named Wirefly. If you lose the flip, destroy all permanents named Wirefly.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: flip_for_wirefly,
        }),
    )
}

fn flip_for_wirefly(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wirefly = reg.interner().lookup("Wirefly").unwrap_or_default();
    let insect = reg.interner().lookup("Insect").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    let token = TokenDefinition {
        name: wirefly,
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    let named_wirefly = ObjectFilter {
        name: reg.interner().lookup("Wirefly"),
        ..ObjectFilter::default()
    };
    let ids = script::ids_matching(state, &named_wirefly, ctx.controller);
    vec![Effect::FlipCoin {
        player: ctx.controller,
        win: Box::new(Effect::CreateToken {
            controller: ctx.controller,
            token,
        }),
        lose: Some(Box::new(Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DestroyPermanent {
                target: arcana_core::objects::NULL_OBJECT_ID,
            }),
        })),
    }]
}
