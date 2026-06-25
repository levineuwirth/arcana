//! Bone Dragon — `{3}{B}{B}` 5/4 Creature — Dragon Skeleton.
//!
//! Oracle:
//! * Flying.
//! * `{3}{B}{B}, Exile seven other cards from your graveyard: Return this
//!   card from your graveyard to the battlefield tapped.`
//!
//! The activation cost "exile seven other cards from your graveyard" is wired
//! via `exile_graveyard_other` / `exile_graveyard_count` (mana + exile-from-
//! graveyard cost), and the ability reanimates the source from the graveyard.
//! GAP: the returned card should enter TAPPED — there is no return-from-
//! graveyard-tapped effect variant, so it enters untapped. GAP: the engine's
//! exile-from-graveyard enumeration does not exclude the source, so the "other"
//! qualifier (the seven cards must be other than this card) is not enforced.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bone Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(skeleton);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}{B}, Exile seven other cards from your graveyard: Return this card from your graveyard to the battlefield tapped.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}{B}").unwrap(),
                    exile_graveyard_other: Some(ObjectFilter::new()),
                    exile_graveyard_count: 7,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: reanimate_self,
            }),
    )
}

fn reanimate_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should enter the battlefield TAPPED — no return-from-graveyard-tapped
    // effect variant; enters untapped.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
